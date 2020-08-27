use std::char;

const MIN_SIZE: usize = 2;
const MAX_SIZE: usize = 3;

#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
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

fn make_empty_boards(size: usize) -> Vec<Board> {
    let mut output = Vec::new();
    let combinations = 1 << (size * size);

    for seed in 0..combinations {
        let mut board = Vec::new();
        
        for y in 0..size {
            let mut row = Vec::new();
            for x in 0..size {
                let index = y * size + x;
                let mask = (seed >> index) & 1;
                let cell = if mask == 1 { Cell::Floor } else { Cell::Wall };
                row.push(cell);
            }
            board.push(row);
        }
        output.push(board);
    }
    return output;
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

fn rotate_board(board: &Board) -> Board {
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

fn all_rotations(board: &Board) -> Vec<Board> {
    let rot0 = board.clone();
    let rot90  = rotate_board(&rot0);
    let rot180 = rotate_board(&rot90);
    let rot270 = rotate_board(&rot180);

    return vec![rot0, rot90, rot180, rot270];
}

fn remove_rotations(input: &Vec<Board>) -> Vec<Board> {
    let mut output   : Vec<Board>      = Vec::new();
    let mut rotations: Vec<Vec<Board>> = Vec::new();

    'candidates: for candidate in input {
        for set in &rotations {
            for test in set {
                if candidate == test {
                    continue 'candidates;
                }
            }
        }
        output.push(candidate.clone());
        rotations.push(all_rotations(candidate));
    }

    return output;
}

fn reflect_y(board: &Board) -> Board {
    let size = board.len();
    let mut output = Vec::new();
        
    for y in 0..size {
        let mut row = Vec::new();
        for x in 0..size {
            row.push(board[size - 1 - y][x]);
        }
        output.push(row);
    }

    return output;
}

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

/// Do the normal diagonal transpose as a copy.
fn transpose(board: &Board) -> Board {
    let size = board.len();
    let mut output = Vec::new();
        
    for y in 0..size {
        let mut row = Vec::new();
        for x in 0..size {
            row.push(board[x][y]);
        }
        output.push(row);
    }

    return output;
}

/// Do the other diagonal symmetry.
fn other_transpose(board: &Board) -> Board {
    let size = board.len();
    let mut output = Vec::new();
        
    for y in 0..size {
        let mut row = Vec::new();
        for x in 0..size {
            row.push(board[size - 1- x][size - 1 - y]);
        }
        output.push(row);
    }

    return output;
}

fn all_reflections(board: &Board) -> Vec<Board> {
    return vec![board.clone(), reflect_y(board), reflect_x(board), transpose(board), other_transpose(board)];
}

fn remove_reflections(input: &Vec<Board>) -> Vec<Board> {
    let mut output     : Vec<Board>      = Vec::new();
    let mut reflections: Vec<Vec<Board>> = Vec::new();

    'candidates: for candidate in input {
        for set in &reflections {
            for test in set {
                if candidate == test {
                    continue 'candidates;
                }
            }
        }
        output.push(candidate.clone());
        reflections.push(all_reflections(candidate));
    }

    return output;
}

/// Remove duplicates (under rotation and reflection) from a list of boards.
fn clean_list(input: &Vec<Board>) -> Vec<Board> {
    return remove_reflections(&remove_rotations(input));
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
        let neighbors: Vec<(i32, i32)> = vec![(ii - 1, jj), (ii + 1, jj), (ii, jj - 1), (ii, jj + 1)];
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

/// Returns a copy of the input board, with only one connected component preserved (prefers biggest).
fn remove_components(board: &Board) -> Board {
    let mut output = board.clone();
    let size = board.len();

    let cell_counts = connected_components(&mut output);
    if cell_counts.is_empty() {
        return output;
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
            if let Cell::Zone(idx) = output[y][x] {
                if idx == max_idx as u32 {
                    output[y][x] = Cell::Floor;
                } else {
                    output[y][x] = Cell::Wall;
                }
            }
        }
    }

    return output;
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

fn remove_small(list: &Vec<Board>) -> Vec<Board> {
    let mut output = Vec::new();

    for board in list {
        if count_cells(board, Cell::Floor) > 2 {
            output.push(board.clone());
        }
    }

    return output;
}

fn main() {

    for size in MIN_SIZE..=MAX_SIZE {
        let raw_empty_boards = make_empty_boards(size);
        let connected_empty_boards = raw_empty_boards.iter().map(remove_components).collect();
        let trimmed_empty_boards = remove_small(&clean_list(&connected_empty_boards));
 
        for empty_board in trimmed_empty_boards {
        // for empty_board in trimmed_empty_boards {
            println!("================================");

            println!("-------- EMPTY BOARD --------");
            print_board(&empty_board);

            println!("-------- FILLED BOARDS --------");
            let raw_filled_boards = add_box_goal(&empty_board);
            let filled_boards = clean_list(&raw_filled_boards);
            for filled_board in filled_boards {
                println!("--------");
                print_board(&filled_board);
            }
        }
    }
}
