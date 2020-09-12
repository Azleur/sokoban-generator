use crate::base::Board;
use crate::tools::symmetry;

/// Accumulating filter. Discards boards if they correspond to a symmetry of a previously accepted board.
pub struct Symmetries {
    /// Source iterator.
    source: Box<dyn Iterator<Item = Board>>,
    /// All boards that have been accepted by the filter.
    boards: Vec<Board>,
    /// D4 symmetries of the boards accepted by the filter. symmetries[i] corresponds to boards[i], etc.
    symmetries: Vec<Vec<Board>>,
}

impl Symmetries {
    pub fn new(source: Box<dyn Iterator<Item = Board>>) -> Symmetries {
        return Symmetries {
            source: source,
            boards: Vec::new(),
            symmetries: Vec::new(),
        };
    }
}

impl Iterator for Symmetries {
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
