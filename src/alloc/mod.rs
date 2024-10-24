use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;

trait Alloc {
    fn alloc<T>(&self, object: T) -> *const T;
}

pub type BlockAddr = NonNull<u8>;
pub type BlockSize = usize;

pub struct Block {
    addr: BlockAddr,
    size: BlockSize,
}

pub enum BlockError {
    InvalidSize,
    OOM,
}

impl Block {
    pub fn new(size: BlockSize) -> Result<Self, BlockError> {
        if !size.is_power_of_two() {
            return Err(BlockError::InvalidSize);
        }

        unsafe {
            let layout = Layout::from_size_align_unchecked(size, size);
            let ptr = alloc(layout);

            if ptr.is_null() {
                return Err(BlockError::OOM);
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

pub const BLOCK_SIZE_BITS: usize = 15;
pub const BLOCK_SIZE: usize = 1 << BLOCK_SIZE_BITS;

pub struct BumpBlock {
    cursor: *const u8,
    limit: *const u8,
    block: Block,
    meta: BlockMeta,
}

impl BumpBlock {
    pub fn inner_alloc(&mut self, alloc_size: usize) -> Option<*const u8> {
        let block_start_ptr = self.block.as_ptr() as usize;
        let cursor_ptr = self.cursor as usize;

        // align to word boundary
        let align_mask: usize = !(std::mem::size_of::<usize>() - 1);

        let next_ptr = cursor_ptr.checked_sub(alloc_size)? & align_mask;

        if next_ptr < block_start_ptr {
            return None;
        }

        self.cursor = next_ptr as *const u8;
        Some(next_ptr)
    }
}
