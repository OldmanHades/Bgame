// importing

use std::io::{self, Write};
use rand::Rng;


// Define size of the game board as a constant
const BOARD_SIZE: usize = 10;

struct Board{
    grid: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    ships: Vec<(usize, usize)>,
}

#[derive(Clone, Copy, PartialEq)]
enum CellState{
    Empty,
    Ship,
    Hit,
    Miss
}

impl Board{
    fn new() -> Self{
        Board{
            grid: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            ships: Vec::new(),
        }
    }
    // Function to randomly place ships on the board
    fn place_ships(&mut self, size: usize) {
        let mut rng = rand::thread_rng();
        loop {
            let row = rng.gen_range(0..BOARD_SIZE);
            let col = rng.gen_range(0..BOARD_SIZE);
            let direction: bool = rng.gen(); // true: horizontal, false: vertical
            if self.can_place_ship(row, col, direction, size) {
                // Place ship cells
                for i in 0..size {
                    let (r, c) = if direction {
                        (row, col + i)
                    } else {
                        (row + i, col)
                    };
                    self.grid[r][c] = CellState::Ship;
                    self.ships.push((r, c));
                }
                break;
            }
        }
    }
    // Check if a ship of a given size can be placed at (row, col) in the given direction
    fn can_place_ship(&self, row: usize, col: usize, direction: bool, size: usize) -> bool {
        if direction {
            // Horizontal
            if col + size > BOARD_SIZE {
                return false;
            }
            for i in 0..size {
                if self.grid[row][col + i] != CellState::Empty {
                    return false;
                }
            }
        } else {
            // Vertical
            if row + size > BOARD_SIZE {
                return false;
            }
            for i in 0..size {
                if self.grid[row + i][col] != CellState::Empty {
                    return false;
                }
            }
        }
        true
    }
    // Method to process a shot at (row, col) and return the resulting CellState
    fn fire(&mut self, row: usize, col: usize) -> CellState {
        match self.grid[row][col] {
            CellState::Empty => {
                self.grid[row][col] = CellState::Miss;
                CellState::Miss
            },
            CellState::Ship => {
                self.grid[row][col] = CellState::Hit;
                CellState::Hit
            },
            CellState::Hit | CellState::Miss => self.grid[row][col], // Already hit or missed
        }
    }
    // Method to Display Game Board
    fn display(&self, hide_ships: bool) {
        // Print column headers
        print!("   ");
        for i in 0..BOARD_SIZE {
            print!("{:2} ", i);
        }
        println!();
        // Print each row
        for (i, row) in self.grid.iter().enumerate() {
            print!("{:2} ", i);
            for cell in row {
                match cell {
                    CellState::Empty => print!(" \u{25A0} "),
                    CellState::Ship => {
                        if hide_ships {
                            print!(" \u{25A0} ");
                        } else {
                            print!(" \u{25B2} ");
                        }
                    }
                    CellState::Hit => print!(" \u{274C} "),
                    CellState::Miss => print!(" \u{274E} "),
                }
            }
            println!();
        }
    }
    // Method for Game Over
    fn is_game_over(&self) -> bool {
        self.ships.iter().all(|(r, c)| self.grid[*r][*c] == CellState::Hit)
    }
}

// Function to get and validate player input (row, col)
fn get_player_input() -> (usize, usize) {
    loop {
        print!("Enter your shot (row col): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 2 {
            println!("Please enter two numbers (row and column).");
            continue;
        }
        if let (Ok(row), Ok(col)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
            if row < BOARD_SIZE && col < BOARD_SIZE {
                return (row, col);
            } else {
                println!("Coordinates out of bounds. Try again.");
            }
        } else {
            println!("Invalid input. Please enter numbers.");
        }
    }
}

// Function to generate a random valid move for the opponent
fn generate_opponent_move(board: &Board) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let mut attempts = 0;
    loop {
        let row = rng.gen_range(0..BOARD_SIZE);
        let col = rng.gen_range(0..BOARD_SIZE);
        // Only pick cells that have not been hit or missed yet
        if board.grid[row][col] == CellState::Empty || board.grid[row][col] == CellState::Ship {
            return (row, col);
        }
        attempts += 1;
        if attempts > 100 { // Prevent infinite loops
            // Fallback: find first available cell
            for row in 0..BOARD_SIZE {
                for col in 0..BOARD_SIZE {
                    if board.grid[row][col] == CellState::Empty || board.grid[row][col] == CellState::Ship {
                        return (row, col);
                    }
                }
            }
            // If all cells are already hit/missed (shouldn't happen if game_over is checked properly)
            panic!("No valid moves left!");
        }
    }
}



// Main function to run the Battleship game
fn main() {
    // Number of ships for each player (could be made configurable)
    let ship_sizes = [5, 4, 3, 3, 2]; // Standard Battleship ship sizes
    // Initialize boards for player and computer
    let mut player_board = Board::new();
    let mut computer_board = Board::new();
    // Place ships for both players
    for &size in &ship_sizes {
        player_board.place_ships(size);
        computer_board.place_ships(size);
    }
    println!("Welcome to Battleship!");
    // Main game loop
    loop {
        println!("\nYour Board:");
        player_board.display(false);
        println!("\nOpponent's Board:");
        computer_board.display(true);
        // Player's turn
        println!("\nYour turn:");
        let (row, col) = get_player_input();
        let result = computer_board.fire(row, col);
        match result {
            CellState::Hit => println!("Hit!"),
            CellState::Miss => println!("Miss!"),
            _ => println!("Already targeted. Try again next turn."),
        }
        if computer_board.is_game_over() {
            println!("Congratulations! You sunk all the enemy ships!");
            break;
        }
        // Computer's turn
        println!("\nOpponent's turn:");
        let (row, col) = generate_opponent_move(&player_board);
        let result = player_board.fire(row, col);
        match result {
            CellState::Hit => println!("Opponent hit your ship at ({}, {})!", row, col),
            CellState::Miss => println!("Opponent missed at ({}, {})!", row, col),
            _ => (),
        }
        if player_board.is_game_over() {
            println!("Game over! The opponent sunk all your ships.");
            break;
        }
    }
}