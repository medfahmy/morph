use std::alloc::{alloc, dealloc, Layout};
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub const BLOCK_SIZE_BITS: usize = 15;
pub const BLOCK_SIZE: usize = 1 << BLOCK_SIZE_BITS; // 32kb
pub const LINE_SIZE_BITS: usize = 7;
pub const LINE_SIZE: usize = 1 << LINE_SIZE_BITS; // 128b
pub const LIVE_COUNT: usize = BLOCK_SIZE / LINE_SIZE; // 256
pub const BLOCK_CAPACITY: usize = BLOCK_SIZE - LIVE_COUNT;
pub const ALIGN_MASK: usize = 1;

trait Alloc {
    fn alloc<T>(&self, object: T) -> *const T;
}

pub type BlockAddr = NonNull<u8>;
pub type BlockSize = usize;

pub struct Block {
    addr: BlockAddr,
    size: BlockSize,
}

pub enum AllocError {
    InvalidSize,
    OOM,
}

impl Block {
    pub fn new(size: BlockSize) -> Result<Self, AllocError> {
        if !size.is_power_of_two() {
            return Err(AllocError::InvalidSize);
        }

        unsafe {
            let layout = Layout::from_size_align_unchecked(size, size);
            let ptr = alloc(layout);

            if ptr.is_null() {
                return Err(AllocError::OOM);
            }

            // address & alignment (size - 1) should be a mutually exclusive set of bits
            let mask = size - 1;
            assert!((ptr as usize & mask) ^ mask == mask);

            Ok(Self {
                addr: NonNull::new_unchecked(ptr),
                size,
            })
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.addr.as_ptr()
    }

    pub fn drop(mut self) {
        unsafe {
            let layout = Layout::from_size_align_unchecked(self.size, self.size);
            dealloc(self.addr.as_mut(), layout);
        }
    }
}

pub struct BumpBlock {
    cursor: *const u8,
    limit: *const u8,
    block: Block,
    meta: BlockMeta,
}

impl BumpBlock {
    pub fn new() -> Result<Self, AllocError> {
        todo!()
    }

    pub fn inner_alloc(&mut self, size: usize) -> Option<*const u8> {
        let ptr = self.cursor as usize;
        let limit = self.limit as usize;
        let next_ptr = ptr.checked_sub(size)? & ALIGN_MASK;

        if next_ptr > limit {
            self.cursor = next_ptr as *const u8;
            return Some(self.cursor);
        }

        let block_relative_limit = unsafe { self.limit.sub(self.block.as_ptr() as usize) } as usize;

        if block_relative_limit > 0 {
            if let Some((cursor, limit)) = self.meta.find_next_gap(block_relative_limit, size) {
                self.cursor = unsafe { self.block.as_ptr().add(cursor) };
                self.limit = unsafe { self.block.as_ptr().add(limit) };
                return self.inner_alloc(size);
            }
        }

        None
    }
}

unsafe fn write<T>(dest: *const u8, object: T) {
    write(dest, object);
}

pub struct BlockMeta {
    lines: *mut u8,
}

impl BlockMeta {
    // locate a gap of unmarked lines of sufficient size to allocate an object
    pub fn find_next_gap(&self, start: usize, size: usize) -> Option<(usize, usize)> {
        let mut count = 0;

        let start_line = start / LINE_SIZE;
        let lines_required = (size + LINE_SIZE - 1) / LINE_SIZE;
        let mut end = start_line;

        for index in (0..start_line).rev() {
            let marked = unsafe { *self.lines.add(index) };

            if marked == 0 {
                count += 1;

                if index == 0 && count >= lines_required {
                    let limit = index * LINE_SIZE;
                    let cursor = end * LINE_SIZE;
                    return Some((cursor, limit));
                }
            } else {
                if count > lines_required {
                    let limit = (index + 2) * LINE_SIZE;
                    let cursor = end * LINE_SIZE;
                    return Some((cursor, limit));
                }

                count = 0;
                end = index;
            }
        }

        None
    }
}

pub struct BlockList {
    head: Option<BumpBlock>,
    overflow: Option<BumpBlock>,
    rest: Vec<BumpBlock>,
}

impl BlockList {
    fn overflow_alloc(&mut self, size: usize) -> Result<*const u8, AllocError> {
        let space = match self.overflow {
            Some(ref mut overflow) => match overflow.inner_alloc(size) {
                Some(space) => space,
                None => {
                    let previous = std::mem::replace(overflow, BumpBlock::new()?);
                    self.rest.push(previous);
                    overflow.inner_alloc(size).expect("Unexpected error")
                }
            },
            None => {
                let mut overflow = BumpBlock::new()?;
                let space = overflow
                    .inner_alloc(size)
                    .expect("Object doesn't fit in block");
                self.overflow = Some(overflow);
                space
            }
        };

        Ok(space)
    }
}

pub struct Heap<H> {
    blocks: UnsafeCell<BlockList>,
    _header_type: PhantomData<*const H>,
}

pub struct SizeClass {}

impl<H> Heap<H> {
    pub fn find_space(&self, size: usize, size_class: SizeClass) -> Result<*const u8, AllocError> {
        let blocks = unsafe { &mut *self.blocks.get() };

        todo!()
    }
}

pub trait AllocTypeId: Copy + Clone {}
pub trait AllocObject<TypeId> {}
pub struct Mark {}
pub struct ArraySize {}

pub trait AllocHeader: Sized {
    type TypeId: AllocTypeId;

    fn new<O: AllocObject<Self::TypeId>>(size: u32, size_class: SizeClass, mark: Mark) -> Self;
    fn new_array(size: ArraySize, size_class: SizeClass, mark: Mark) -> Self;
    fn mark(&mut self);
    fn is_marked(&self) -> bool;
    fn size_class(&self) -> SizeClass;
    fn size(&self) -> u32;
    fn type_id(&self) -> Self::TypeId;
}
