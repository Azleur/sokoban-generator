use crate::base::Board;
use crate::tools::symmetry;

/// Accumulating filter. Discards boards if they correspond to a symmetry of a previously accepted board.
pub struct Symmetries {
    /// All boards that have been accepted by the filter.
    boards: Vec<Board>,
    /// D4 symmetries of the boards accepted by the filter. symmetries[i] corresponds to boards[i], etc.
    symmetries: Vec<Vec<Board>>,
}

impl Symmetries {
    pub fn new() -> Symmetries {
        return Symmetries {
            boards: Vec::new(),
            symmetries: Vec::new(),
        };
    }

    /// If board or one of its D4 symmetries has been seen before, return false. Otherwise, return true and update the registry.
    pub fn accept(&mut self, board: &Board) -> bool {
        for set in &self.symmetries {
            for test in set {
                if test == board {
                    return false;
                }
            }
        }
        self.boards.push(board.clone());
        self.symmetries.push(symmetry::all(&board));
        return true;
    }
}
