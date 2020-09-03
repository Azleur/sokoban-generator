//! Flood fill implementation and utilities for boards.

use crate::base::{Board, Cell};

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
pub fn remove_components(board: &mut Board) {
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
