use crate::base::{Cell, Board, is_wall, is_goal, find_cell};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction { Up, Right, Down, Left }

/// Object returned by move_piece(), summarizing board status information.
pub struct MoveStats {
    /// Number of cells the piece traveled.
    pub cells_moved: u8,
    /// (x,y) indices for the piece position, after moving.
    pub piece_pos: (u8, u8),
    /// True if this move won the game (i.e., moved the piece into the goal).
    pub victory: bool,
}

/// Try moving the piece in the given direction, and return stats for the board status after moving.
///
/// Returns None if there is no piece to move; Some(stats) otherwise.
pub fn move_piece(board: &mut Board, direction: Direction) -> Option<MoveStats> {
    let size = board.len() as u8;

    if let Some(init_pos) = find_cell(board, Cell::Piece) {
        let delta: (i32, i32) = match direction {
            Direction::Up    => ( 0, -1),
            Direction::Right => ( 1,  0),
            Direction::Down  => ( 0,  1),
            Direction::Left  => (-1,  0),
        };
        let mut pos = init_pos;
        let mut num_cells = 0;
        let mut victory = false;

        loop {
            let new_pos = ((pos.0 as i32 + delta.0) as u8, (pos.1 as i32 + delta.1) as u8);

            // NOTE: This depends on uint overflow to work correctly!
            if new_pos.0 >= size || new_pos.1 >= size {
                break;
            }

            let cell = board[new_pos.1 as usize][new_pos.0 as usize];

            if is_wall(cell) {
                break;
            }

            pos = new_pos;
            num_cells += 1;

            if is_goal(cell) {
                victory = true;
                break;
            }
        }

        board[init_pos.1 as usize][init_pos.0 as usize] = Cell::Floor;
        board[pos.1 as usize][pos.0 as usize] = Cell::Piece;

        return Some(MoveStats { cells_moved: num_cells, piece_pos: pos, victory: victory});
    }
    
    return None;
}

/// Summary of recursive exploration of the move space of a board.
#[derive(Default, Debug)]
pub struct ExploreStats {
    /// Can this board be solved?
    pub solvable: bool,
    pub num_moves: usize,
    pub solution: Vec<Direction>,
}

/// Tree structure around a sequence of moves.
struct TreeNode {
    board: Board,
    reached_by: Option<Direction>,
    parent_idx: Option<usize>,
}

struct Tree {
    nodes: Vec<TreeNode>,
}

impl Tree {
    /// Create a new tree from a board.
    fn new(board: Board) -> Tree {
        Tree {
            nodes: vec![TreeNode {
                board: board, 
                reached_by: None, 
                parent_idx: None
            }],
        }
    }

    fn get(&self, idx: usize) -> &TreeNode {
        &self.nodes[idx]
    }

    fn push(&mut self, node: TreeNode) {
        self.nodes.push(node)
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Check if self contains a given board in one of its nodes.
    fn contains_board(&self, board: &Board) -> bool {
        for node in &self.nodes {
            if node.board == *board {
                return true;
            }
        }
        return false;
    }

    /// Unwind the moves needed to obtain a given board position.
    fn trace_moves(&self, idx: usize) -> Vec<Direction> {
        let mut out: Vec<Direction> = Vec::new();
        let mut node = self.get(idx);
    
        while let Some(dir) = &node.reached_by {
            out.push(*dir);
            match node.parent_idx {
                Some(parent_idx) => node = self.get(parent_idx),
                None => break,
            }
        }
    
        out.reverse();
        return out;
    }
}

/// Recursively explore all possible moves.
pub fn explore_space(board: &Board) -> ExploreStats {
    if let None = find_cell(board, Cell::Piece) {
        return Default::default();
    }

    let mut tree = Tree::new(board.clone());
    let mut idx = 0;

    while idx < tree.len() {
        for dir in &[Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            let node = tree.get(idx);
            if Some(*dir) == node.reached_by {
                continue;
            }

            let mut candidate_board = node.board.clone();
            let stats = move_piece(&mut candidate_board, *dir).unwrap();

            if stats.cells_moved == 0 || tree.contains_board(&candidate_board) {
                continue;
            }

            let new_node = TreeNode { 
                board: candidate_board, 
                reached_by: Some(*dir), 
                parent_idx: Some(idx), 
            };
            tree.push(new_node);
            let new_idx = &tree.len() - 1;

            if stats.victory {
                let solution = tree.trace_moves(new_idx);
                return ExploreStats { 
                    solvable: true, 
                    num_moves: solution.len(),
                    solution: solution,
                };
            }
        }

        idx += 1;
    }

    return Default::default();
}