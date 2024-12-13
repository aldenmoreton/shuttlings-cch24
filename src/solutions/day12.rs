use std::{fmt::Display, sync::Arc};

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension};
use itertools::Itertools;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Cell {
    #[default]
    Empty,
    Cookie,
    Milk,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "⬛"),
            Cell::Cookie => write!(f, "🍪"),
            Cell::Milk => write!(f, "🥛"),
        }
    }
}

impl Cell {
    fn winner(&self) -> String {
        match self {
            Cell::Cookie => "🍪 wins!",
            Cell::Milk => "🥛 wins!",
            Cell::Empty => "No winner.",
        }
        .to_string()
    }
}

#[derive(Debug, Default)]
pub enum Status {
    #[default]
    InProgress,
    Winner(Cell),
}

#[derive(Default)]
pub struct Board([[Cell; 4]; 4], Status);

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            write!(f, "⬜")?;
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f, "⬜")?;
        }
        writeln!(f, "⬜⬜⬜⬜⬜⬜")?;

        match &self.1 {
            Status::InProgress => (),
            Status::Winner(winner) => {
                writeln!(f, "{}", winner.winner())?;
            }
        }

        Ok(())
    }
}

impl Board {
    fn insert(&mut self, team: Cell, col: usize) -> Option<usize> {
        let board = &mut self.0;

        for (i, row) in board.iter_mut().enumerate().rev() {
            let next_cell = &mut row[col];
            if *next_cell == Cell::Empty {
                *next_cell = team;
                return Some(i);
            }
        }

        None
    }

    fn check(&self, row: usize, col: usize) -> Status {
        let board = &self.0;

        if board[row][0] != Cell::Empty && board[row].iter().all_equal() {
            return Status::Winner(board[row][col].clone());
        }
        if board[0][col] != Cell::Empty && board.iter().map(|row| &row[col]).all_equal() {
            return Status::Winner(board[row][col].clone());
        }
        if row == col
            && board[0][0] != Cell::Empty
            && board.iter().enumerate().map(|(i, row)| &row[i]).all_equal()
        {
            return Status::Winner(board[row][col].clone());
        }

        if row + col == 3
            && board[0][3] != Cell::Empty
            && (0..4)
                .rev()
                .zip(board.iter())
                .map(|(i, row)| &row[i])
                .all_equal()
        {
            return Status::Winner(board[row][col].clone());
        }

        if row == 0 && !board[0].iter().any(|cell| *cell == Cell::Empty) {
            return Status::Winner(Cell::Empty);
        }

        Status::InProgress
    }
}

type BoardLock = Extension<Arc<RwLock<Board>>>;

pub async fn board(Extension(board): BoardLock) -> String {
    let board = board.read().await;
    println!("Printing board");
    board.to_string()
}

pub async fn reset(Extension(board): BoardLock) -> String {
    println!("reset");
    let mut board = board.write().await;
    *board = Default::default();
    board.to_string()
}

pub async fn place(
    Path((team, column)): Path<(String, usize)>,
    Extension(board): BoardLock,
) -> impl IntoResponse {
    println!("Placing: {column}");
    if !matches!(column, 1..=4) {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let column = column - 1;
    let mut board = board.write().await;
    match board.1 {
        Status::Winner(_) => {
            return (StatusCode::SERVICE_UNAVAILABLE, board.to_string()).into_response()
        }
        Status::InProgress => (),
    }

    let team_cell = match team.as_str() {
        "cookie" => Cell::Cookie,
        "milk" => Cell::Milk,
        _ => return StatusCode::BAD_REQUEST.into_response(),
    };

    if let Some(row) = board.insert(team_cell, column) {
        let new_status = board.check(row, column);
        println!("new status: {new_status:?}");
        board.1 = new_status;
        board.to_string().into_response()
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, board.to_string()).into_response()
    }
}
