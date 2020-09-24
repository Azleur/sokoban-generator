use crate::base::{Board, BaseIter, SecondaryIter, WrapperIter};
use crate::play::explore_space;

/// Wrapper around a Board iterator. Ensures the returned board is solvable.
pub struct Solvable<T: Iterator<Item = Board>> {
    /// Source iterator.
    source: T,
}

impl<T: Iterator<Item = Board>> WrapperIter<T> for Solvable<T> {
    fn new_wrapper(source: T) -> Self {
        Solvable {
            source: source,
        }
    }
}

impl<T: BaseIter> BaseIter for Solvable<T> {
    fn new_base(size: u8) -> Self {
        Solvable {
            source: T::new_base(size),
        }
    }
}

impl<T: SecondaryIter> SecondaryIter for Solvable<T> {
    fn new_secondary(board: Board) -> Self {
        Solvable {
            source: T::new_secondary(board),
        }
    }
}

impl<T: Iterator<Item = Board>> Iterator for Solvable<T> {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(board) = self.source.next() {
            let stats = explore_space(&board);
            if stats.solvable {
                return Some(board);
            }
        }

        return None;
    }
}
