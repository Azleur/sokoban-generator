use std::char;

const MIN_SIZE: usize = 5;
const MAX_SIZE: usize = 5;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Floor,
    Wall,
    Piece,
    Goal,
    Zone(u32),
}

fn cell_symbol(cell: &Cell) -> char {
    match cell {
        Cell::Floor => '.',
        Cell::Wall => 'X',
        Cell::Piece => '#',
        Cell::Goal => '@',
        Cell::Zone(num) => char::from_digit(*num, 10).unwrap(),
    }
}

type Board = Vec<Vec<Cell>>;

fn print_board(board: &Board) {
    for row in board {
        for cell in row {
            print!("{} ", cell_symbol(cell));
        }
        print!("\n");
    }
}

/// Makes all  unique empty boards of the given size.
struct BoardMaker {
    /// Next value used to determine the pattern of floor and wall tiles. Range [0, self.combinations).
    seed: usize,
    /// Number of rows (or columns) in each board.
    size: usize,
    /// Total number of floor & wall combinations possible in this board size (2 ^ (size * size)).
    combinations: usize,
    /// Used to remove duplicates under symmetry.
    filter: BoardFilter,
}

impl BoardMaker {
    fn new(size: usize) -> BoardMaker {
        return BoardMaker {
            seed: 0,
            size: size,
            combinations: 1 << (size * size),
            filter: BoardFilter::new(),
        };
    }
}

impl Iterator for BoardMaker {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        while self.seed < self.combinations {
            let mut board = make_empty_board(self);
            remove_components(&mut board);
            self.seed += 1;

            if count_cells(&board, Cell::Floor) > 2 && self.filter.accept(&board) {
                return Some(board);
            }
        }

        return None;
    }
}

/// Accumulating filter. Discards boards if they correspond to a symmetry of a previously accepted board.
struct BoardFilter {
    /// All boards that have been accepted by the filter.
    boards: Vec<Board>,
    /// D4 symmetries of the boards accepted by the filter. symmetries[i] corresponds to boards[i], etc.
    symmetries: Vec<Vec<Board>>,
}

impl BoardFilter {
    fn new() -> BoardFilter {
        return BoardFilter {
            boards: Vec::new(),
            symmetries: Vec::new(),
        };
    }

    /// If board or one of its D4 symmetries has been seen before, return false. Otherwise, return true and update the registry.
    fn accept(&mut self, board: &Board) -> bool {
        for set in &self.symmetries {
            for test in set {
                if test == board {
                    return false;
                }
            }
        }
        self.boards.push(board.clone());
        self.symmetries.push(all_symmetries(&board));
        return true;
    }
}

fn make_empty_board(maker: &BoardMaker) -> Board {
    let mut board = Vec::new();

    for y in 0..maker.size {
        let mut row = Vec::new();
        for x in 0..maker.size {
            let index = y * maker.size + x;
            let mask = (maker.seed >> index) & 1;
            let cell = if mask == 1 { Cell::Floor } else { Cell::Wall };
            row.push(cell);
        }
        board.push(row);
    }

    return board;
}

fn add_box_goal(original: &Board) -> Vec<Board> {
    let mut output = Vec::new();

    let mut floor_indices: Vec<(usize, usize)> = Vec::new();
    for j in 0..original.len() {
        for i in 0..original[j].len() {
            if let Cell::Floor = original[j][i] {
                floor_indices.push((i, j));
            }
        }
    }

    if floor_indices.len() < 2 {
        return output;
    }

    // for each possible combination add box and goal.
    for box_idx in &floor_indices {
        for goal_idx in &floor_indices {
            if box_idx != goal_idx {
                let mut board = original.clone();

                let (box_i, box_j) = *box_idx;
                board[box_j][box_i] = Cell::Piece;

                let (goal_i, goal_j) = *goal_idx;
                board[goal_j][goal_i] = Cell::Goal;

                output.push(board);
            }
        }
    }

    return output;
}

/// Counter-clockwise 90° rotation.
fn rotate(board: &Board) -> Board {
    let size = board.len();
    let mut output = Vec::new();
    for y in 0..size {
        let mut row = Vec::new();
        for x in 0..size {
            row.push(board[x][size - 1 - y]);
        }
        output.push(row);
    }

    return output;
}

/// Reflect over the vertical axis x = size / 2.
fn reflect_x(board: &Board) -> Board {
    let size = board.len();
    let mut output = Vec::new();
    for y in 0..size {
        let mut row = Vec::new();
        for x in 0..size {
            row.push(board[y][size - 1 - x]);
        }
        output.push(row);
    }

    return output;
}

/// Calculate all D4 (dihedral 4) symmetries of a board.
fn all_symmetries(board: &Board) -> Vec<Board> {
    let e = board.clone();
    let a = rotate(board);
    let a2 = rotate(&a);
    let a3 = rotate(&a2);
    let b = reflect_x(board);
    let ab = rotate(&b);
    let a2b = rotate(&ab);
    let a3b = rotate(&a2b);

    return vec![e, a, a2, a3, b, ab, a2b, a3b];
}

/// Given an initial Floor position in a board and a Zone tag, mark the connected component.
fn paint_component(board: &mut Board, x: usize, y: usize, component: u32) -> usize {
    let size = board.len() as i32;
    let cell_type = Cell::Zone(component);
    let mut pending = vec![(x, y)];
    let mut cell_count = 0;

    while let Some((i, j)) = pending.pop() {
        board[j][i] = cell_type;
        cell_count += 1;
        let ii = i as i32;
        let jj = j as i32;
        let neighbors: Vec<(i32, i32)> =
            vec![(ii - 1, jj), (ii + 1, jj), (ii, jj - 1), (ii, jj + 1)];
        for (kk, ll) in neighbors {
            if kk >= 0 && ll >= 0 && kk < size && ll < size {
                let k = kk as usize;
                let l = ll as usize;
                if let Cell::Floor = board[l][k] {
                    pending.push((k, l));
                }
            }
        }
    }

    return cell_count;
}

/// Change the Floor tiles in a board for Zone tiles, indicating connected components.
fn connected_components(board: &mut Board) -> Vec<usize> {
    let size = board.len();
    let mut component_count = 0;
    let mut cell_counts = Vec::new();
    for y in 0..size {
        for x in 0..size {
            if let Cell::Floor = board[y][x] {
                let cell_count = paint_component(board, x, y, component_count);
                component_count += 1;
                cell_counts.push(cell_count);
            }
        }
    }

    return cell_counts;
}

/// Remove all connected components except biggest.
fn remove_components(board: &mut Board) {
    let size = board.len();

    let cell_counts = connected_components(board);
    if cell_counts.is_empty() {
        return;
    }

    let mut max_count: usize = 0;
    let mut max_idx: usize = 0;
    for (idx, count) in cell_counts.iter().enumerate() {
        if *count > max_count {
            max_idx = idx;
            max_count = *count;
        }
    }

    for y in 0..size {
        for x in 0..size {
            if let Cell::Zone(idx) = board[y][x] {
                if idx == max_idx as u32 {
                    board[y][x] = Cell::Floor;
                } else {
                    board[y][x] = Cell::Wall;
                }
            }
        }
    }
}

fn count_cells(board: &Board, cell: Cell) -> usize {
    let size = board.len();
    let mut count = 0;

    for y in 0..size {
        for x in 0..size {
            if board[y][x] == cell {
                count += 1;
            }
        }
    }

    return count;
}

fn main() {
    for size in MIN_SIZE..=MAX_SIZE {
        println!("================================================================");
        println!(
            "=========================== SIZE {} ============================",
            size
        );
        println!("================================================================");

        for empty_board in BoardMaker::new(size) {
            println!("================================");

            println!("-------- EMPTY BOARD --------");
            print_board(&empty_board);

            println!("-------- FILLED BOARDS --------");
            let mut filter = BoardFilter::new();
            let mut filled_boards = add_box_goal(&empty_board);
            filled_boards.retain(|board| filter.accept(board));
            for filled_board in filled_boards {
                println!("--------");
                print_board(&filled_board);
            }
        }
    }
}
