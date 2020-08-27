use std::char;

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

const MIN_SIZE: usize = 3;
const MAX_SIZE: usize = 3;

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

fn add_box_goal(original: Board) -> Vec<Board> {
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

fn all_reflections(board: &Board) -> Vec<Board> {
    return vec![board.clone(), reflect_y(board), reflect_x(board)];
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

fn clean_list(input: &Vec<Board>) -> Vec<Board> {
    return remove_reflections(&remove_rotations(input));
}

fn paint_component(board: &mut Board, x: usize, y: usize, component: u32) {
    let size = board.len() as i32;
    let cell = Cell::Zone(component);
    let mut pending = vec![(x, y)];

    while let Some((i, j)) = pending.pop() {
        board[j][i] = cell;
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

}

fn connected_components(board: &Board) -> Board {
    let size = board.len();
    let mut output = board.clone();
    let mut component_count = 0;
        
    for y in 0..size {
        for x in 0..size {
            if let Cell::Floor = output[y][x] {
                paint_component(&mut output, x, y, component_count);
                component_count += 1;
            }
        }
    }

    return output;
}

fn main() {

    for size in MIN_SIZE..=MAX_SIZE {
        let all_empty_boards = make_empty_boards(size);
        let trimmed_empty_boards = clean_list(&all_empty_boards);
        let tagged_boards: Vec<Board> = trimmed_empty_boards.iter().map(connected_components).collect();

        for (empty_board, tagged_board) in trimmed_empty_boards.iter().zip(tagged_boards) {
        // for empty_board in trimmed_empty_boards {
            println!("================================");

            println!("-------- EMPTY BOARD --------");
            print_board(&empty_board);

            println!("-------- TAGGED BOARD --------");
            print_board(&tagged_board);

            // println!("-------- FILLED BOARDS --------");
            // let filled_boards = add_box_goal(empty_board);
            // for board in filled_boards {
            //     println!("--------");
            //     print_board(&board);
            // }
        }
    }
}
