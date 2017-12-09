//! Provides an iterator which duplicates items n times

/// An iterator which duplicates each element of another iterator
#[derive(Clone, Debug)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Duplicate<I: Iterator> {
    iter: I,
    duplicates: usize,

    /// Currently duplicated value (duplicates left, value)
    current: Option<(usize, I::Item)>
}

impl<I> Iterator for Duplicate<I> where I: Iterator, I::Item: Clone {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        // Handle stupid duplicates values early
        if self.duplicates == 0 {
            None
        } else if self.duplicates == 1 {
            self.iter.next()
        } else {
            match self.current.take() {
                Some((left, value)) => {
                    // Return our value from current
                    assert!(left > 0);
                    self.current = if left > 1 {
                        Some((left - 1, value.clone()))
                    } else {
                        None
                    };

                    Some(value)
                },
                None => {
                    // We need to fetch a new item from the iterator
                    match self.iter.next() {
                        Some(value) => {
                            self.current = Some((self.duplicates - 1, value.clone()));
                            Some(value)
                        }
                        None => None
                    }
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (iter_lo, iter_hi) = self.iter.size_hint();
        let current_left = self.current.as_ref().map_or(0, |&(last, _)| last);

        (iter_lo * self.duplicates + current_left,
         iter_hi.map(|hi| hi * self.duplicates + current_left))
    }

    fn count(self) -> usize {
        let current_left = self.current.map_or(0, |(last, _)| last);
        self.iter.count() * self.duplicates + current_left
    }
}

/// Returns an iterator which duplicates each element in iter a given number of times
pub fn duplicate<I: Iterator>(iter: I, duplicates: usize) -> Duplicate<I> where I::Item: Clone {
    Duplicate { iter: iter, duplicates: duplicates, current: None }
}
