use crate::base::{Board, BaseIter, SecondaryIter, WrapperIter};
use crate::tools::symmetry;

/// Accumulating filter. Discards boards if they correspond to a symmetry of a previously accepted board.
pub struct Symmetries<T: Iterator<Item = Board>> {
    /// Source iterator.
    source: T,
    /// All boards that have been accepted by the filter.
    boards: Vec<Board>,
    /// D4 symmetries of the boards accepted by the filter. symmetries[i] corresponds to boards[i], etc.
    symmetries: Vec<Vec<Board>>,
}

impl<T: Iterator<Item = Board>> WrapperIter<T> for Symmetries<T> {
    fn new_wrapper(source: T) -> Symmetries<T> {
        return Symmetries {
            source: source,
            boards: Vec::new(),
            symmetries: Vec::new(),
        };
    }
}

impl<T: BaseIter> BaseIter for Symmetries<T> {
    fn new_base(size: u8) -> Symmetries<T> {
        let source = T::new_base(size);
        return Symmetries {
            source: source,
            boards: Vec::new(),
            symmetries: Vec::new(),
        };
    }
}

impl<T: SecondaryIter> SecondaryIter for Symmetries<T> {
    fn new_secondary(board: Board) -> Symmetries<T> {
        let source = T::new_secondary(board);
        return Symmetries {
            source: source,
            boards: Vec::new(),
            symmetries: Vec::new(),
        };
    }
}

impl<T: Iterator<Item = Board>> Iterator for Symmetries<T> {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: while let Some(board) = self.source.next() {
            for set in &self.symmetries {
                for test in set {
                    if *test == board {
                        continue 'outer;
                    }
                }
            }

            self.boards.push(board.clone());
            self.symmetries.push(symmetry::all(&board));
            return Some(board);
        }

        return None;
    }
}
