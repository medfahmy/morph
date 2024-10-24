use std::cell::{Cell, UnsafeCell};
use std::fmt::{self, Debug};
use std::iter::FromIterator;
use std::ops::Index;

const FIRST_CHUNK_SIZE: usize = 16;

pub struct List<T> {
    chunks: UnsafeCell<Vec<Vec<T>>>,
    len: Cell<usize>,
}

impl<T> List<T> {
    fn chunks(&self) -> &[Vec<T>] {
        unsafe { &*self.chunks.get() }
    }

    /// In test builds, check all of the unsafe invariants
    ///
    /// In release builds, no-op
    fn check_invariants(&self) {
        #[cfg(test)]
        {
            if self.len.get() > 0 {
                // Correct number of chunks
                assert_eq!(index_chunk(self.len.get() - 1), self.chunks().len() - 1);

                // Every chunk holds enough items
                for chunk_id in 0..self.chunks().len() {
                    assert!(chunk_size(chunk_id) <= self.chunks()[chunk_id].capacity());
                }

                // Intermediate chunks are full
                for chunk_id in 0..self.chunks().len() - 1 {
                    assert_eq!(chunk_size(chunk_id), self.chunks()[chunk_id].len());
                }

                // Last chunk is correct length
                assert_eq!(
                    self.chunks().last().unwrap().len(),
                    self.len.get() - chunk_start(self.chunks().len() - 1)
                );
            } else {
                // No chunks
                assert_eq!(0, self.chunks().len());
            }
        }
    }

    /// Create a new `List`
    pub fn new() -> Self {
        Self {
            chunks: UnsafeCell::new(Vec::new()),
            len: Cell::new(0),
        }
    }

    /// Append an item to the end
    ///
    /// Note that this does not require `mut`.
    pub fn push(&self, item: T) {
        self.check_invariants();

        // Unsafe code alert!
        //
        // Preserve the following invariants:
        // - Only the last chunk may be modified
        // - A chunk cannot ever be reallocated
        // - len must reflect the length
        //
        // Invariants are checked in the check_invariants method
        let mut_chunks = unsafe { &mut *self.chunks.get() };

        let new_index = self.len.get();
        let chunk_id = index_chunk(new_index);

        if chunk_id < mut_chunks.len() {
            // We should always be inserting into the last chunk
            debug_assert_eq!(chunk_id, mut_chunks.len() - 1);

            // Insert into the appropriate chunk
            let chunk = &mut mut_chunks[chunk_id];

            // The chunk must not be reallocated! Save the pre-insertion capacity
            // so we can check it later (debug builds only)
            #[cfg(test)]
            let prev_capacity = chunk.capacity();

            // Do the insertion
            chunk.push(item);

            // Check that the capacity didn't change (debug builds only)
            #[cfg(test)]
            assert_eq!(prev_capacity, chunk.capacity());
        } else {
            // Need to allocate a new chunk

            // New chunk should be the immediate next chunk
            debug_assert_eq!(chunk_id, mut_chunks.len());

            // New chunk must be big enough
            let mut new_chunk = Vec::with_capacity(chunk_size(chunk_id));
            debug_assert!(new_chunk.capacity() >= chunk_size(chunk_id));

            new_chunk.push(item);

            mut_chunks.push(new_chunk);
        }

        // Increment the length
        self.len.set(self.len.get() + 1);

        self.check_invariants();
    }

    /// Get the length of the list
    pub fn len(&self) -> usize {
        self.check_invariants();

        self.len.get()
    }

    /// Get an item from the list, if it is in bounds
    ///
    /// Returns `None` if the `index` is out-of-bounds. Note that you can also
    /// index with `[]`, which will panic on out-of-bounds.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.check_invariants();

        if index >= self.len() {
            return None;
        }

        let chunk_id = index_chunk(index);
        let chunk_start = chunk_start(chunk_id);

        return Some(&self.chunks()[chunk_id][index - chunk_start]);
    }

    /// Get an iterator over the list
    pub fn iter(&self) -> Iter<T> {
        self.check_invariants();

        Iter {
            list: &self,
            index: 0,
        }
    }
}

const fn chunk_size(chunk_id: usize) -> usize {
    // First chunk is FIRST_CHUNK_SIZE, subsequent chunks double each time
    FIRST_CHUNK_SIZE << chunk_id
}

const fn chunk_start(chunk_id: usize) -> usize {
    // This looks like magic, but I promise it works
    // Essentially, each chunk is the size of the sum of all chunks before
    // it. Except that the first chunk is different: it "should" be preceded
    // by a whole list of chunks that sum to its size, but it's not. Therefore,
    // there's a "missing" set of chunks the size of the first chunk, so
    // later chunks need to be updated.
    chunk_size(chunk_id) - FIRST_CHUNK_SIZE
}

const fn index_chunk(index: usize) -> usize {
    // This *is* magic
    floor_log2(index + FIRST_CHUNK_SIZE) - floor_log2(FIRST_CHUNK_SIZE)
}

#[inline]
const fn floor_log2(x: usize) -> usize {
    const BITS_PER_BYTE: usize = 8;

    BITS_PER_BYTE * std::mem::size_of::<usize>() - (x.leading_zeros() as usize) - 1
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<usize> for List<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("List indexed beyond its length")
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let list = Self::new();

        for item in iter {
            list.push(item);
        }

        list
    }
}

impl<'l, T> IntoIterator for &'l List<T> {
    type Item = &'l T;
    type IntoIter = Iter<'l, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &List<T>) -> bool {
        let mut s = self.iter();
        let mut o = other.iter();

        loop {
            match (s.next(), o.next()) {
                (Some(a), Some(b)) if a == b => {}
                (None, None) => return true,
                _ => return false,
            }
        }
    }
}

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_list().entries(self.iter()).finish()
    }
}

pub struct Iter<'l, T> {
    list: &'l List<T>,
    index: usize,
}

impl<'l, T> Iterator for Iter<'l, T> {
    type Item = &'l T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.list.get(self.index);

        self.index += 1;

        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.list.len() - self.index;

        (remaining, Some(remaining))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_iterator() {
        let l: List<i32> = (0..100).collect();

        for i in 0..100 {
            assert_eq!(l[i], i as i32);
        }
    }

    #[test]
    fn iterator() {
        let l: List<i32> = (0..100).collect();
        let mut i1 = l.iter();
        let mut i2 = l.into_iter();

        for item in 0..100 {
            assert_eq!(i1.next(), Some(&item));
            assert_eq!(i2.next(), Some(&item));
        }

        assert_eq!(i1.next(), None);
        assert_eq!(i2.next(), None);
    }

    #[test]
    fn equality() {
        let a = List::new();
        let b = List::new();

        assert_eq!(a, b);

        a.push("foo");

        assert_ne!(a, b);

        b.push("foo");

        assert_eq!(a, b);

        a.push("bar");
        a.push("baz");

        assert_ne!(a, b);
    }

    #[test]
    fn iterator_size_hint() {
        let l: List<i32> = List::new();
        let mut i = l.iter();
        assert_eq!(i.size_hint(), (0, Some(0)));

        l.push(1);
        assert_eq!(i.size_hint(), (1, Some(1)));

        l.push(2);
        assert_eq!(i.size_hint(), (2, Some(2)));

        i.next();
        assert_eq!(i.size_hint(), (1, Some(1)));

        l.push(3);
        assert_eq!(i.size_hint(), (2, Some(2)));

        i.next();
        assert_eq!(i.size_hint(), (1, Some(1)));

        i.next();
        assert_eq!(i.size_hint(), (0, Some(0)));
    }

    #[test]
    fn chunk_sizes_make_sense() {
        assert_eq!(chunk_size(0), FIRST_CHUNK_SIZE);

        let mut index = 0;

        for chunk in 0..20 {
            // Each chunk starts just after the previous one ends
            assert_eq!(chunk_start(chunk), index);
            index += chunk_size(chunk);
        }
    }

    #[test]
    fn index_chunk_matches_up() {
        for index in 0..1_000_000 {
            let chunk_id = index_chunk(index);

            // Each index happens after its chunk start and before its chunk end
            assert!(index >= chunk_start(chunk_id));
            assert!(index < chunk_start(chunk_id) + chunk_size(chunk_id));
        }
    }

    #[test]
    fn empty_list() {
        let n: List<usize> = List::new();

        assert_eq!(n.len(), 0);
        assert_eq!(n.get(0), None);

        let d: List<usize> = List::default();

        assert_eq!(d.len(), 0);
        assert_eq!(d.get(0), None);
    }

    #[test]
    fn thousand_item_list() {
        test_big_list(1_000);
    }

    #[test]
    #[ignore]
    fn million_item_list() {
        test_big_list(1_000_000);
    }

    fn test_big_list(size: usize) {
        let l = List::new();
        let mut refs = Vec::new();

        for i in 0..size {
            assert_eq!(l.len(), i);

            l.push(i);
            refs.push(l[i]);

            assert_eq!(l.len(), i + 1);
        }

        for i in 0..size {
            assert_eq!(Some(&refs[i]), l.get(i));
        }
    }
}
