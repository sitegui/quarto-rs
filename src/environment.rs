use crate::board::*;
use crate::traits;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct State {
    board: [[Option<Piece>; 4]; 4],
    reserve: Piece,
}

impl State {
    fn new() -> Self {
        State {
            board: [[None; 4]; 4],
            reserve: Piece::from(15),
        }
    }
}

impl traits::State for State {
    fn game_depth(&self) -> u16 {
        let mut depth = 0;
        for row in &self.board {
            for cell in row {
                if cell.is_some() {
                    depth += 1;
                }
            }
        }
        depth
    }
}

pub struct Environment {
    state: State,
    available_positions: Vec<Position>,
    available_pieces: Vec<Piece>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            state: State::new(),
            available_positions: Vec::new(),
            available_pieces: Vec::new(),
        }
    }

    /// Return valid actions for the current state
    fn actions(&self) -> Vec<Action> {
        let mut actions =
            Vec::with_capacity(self.available_positions.len() * self.available_pieces.len());
        for &position in &self.available_positions {
            for &piece in &self.available_pieces {
                actions.push(Action { piece, position });
            }
        }
        actions
    }

    /// Return the final reward (if any), checking all lines that cross the given position
    fn final_reward(&self, position: Position) -> Option<f32> {
        fn pos(row: u8, col: u8) -> Position {
            Position { row, col }
        }

        let (r, c) = (position.row, position.col);
        if self.has_common_trait(pos(r, 0), pos(r, 1), pos(r, 2), pos(r, 3))
            || self.has_common_trait(pos(0, c), pos(1, c), pos(2, c), pos(3, c))
            || self.has_common_trait(pos(0, 0), pos(1, 1), pos(2, 2), pos(3, 3))
            || self.has_common_trait(pos(3, 0), pos(2, 1), pos(1, 2), pos(0, 3))
        {
            Some(100.)
        } else if self.available_positions.is_empty() {
            Some(0.)
        } else {
            None
        }
    }

    fn has_common_trait(
        &self,
        pos1: Position,
        pos2: Position,
        pos3: Position,
        pos4: Position,
    ) -> bool {
        match (
            self.state.board[pos1.row as usize][pos1.col as usize],
            self.state.board[pos2.row as usize][pos2.col as usize],
            self.state.board[pos3.row as usize][pos3.col as usize],
            self.state.board[pos4.row as usize][pos4.col as usize],
        ) {
            (Some(p1), Some(p2), Some(p3), Some(p4)) => {
                macro_rules! is_same {
                    ($field:ident) => {
                        p1.$field == p2.$field && p1.$field == p3.$field && p1.$field == p4.$field
                    };
                }

                is_same!(hollow) || is_same!(square) || is_same!(short) || is_same!(black)
            }
            _ => false,
        }
    }

    /// Put the reserve piece at the given position
    fn apply_position(&mut self, position: Position) {
        assert!(remove_item(&mut self.available_positions, &position));
        let Position { row, col } = position;
        self.state.board[row as usize][col as usize] = Some(self.state.reserve);
    }
}

impl traits::Environment for Environment {
    type State = State;
    type Action = Action;

    fn reset(&mut self) -> (State, Vec<Action>) {
        self.state = State::new();
        self.available_positions = (0..16).map(Position::from).collect();
        self.available_pieces = (0..15).map(Piece::from).collect();
        (self.state, self.actions())
    }

    fn step(&mut self, action: Action) -> (State, f32, bool, Vec<Action>) {
        // Apply move
        self.apply_position(action.position);
        assert!(remove_item(&mut self.available_pieces, &action.piece));
        self.state.reserve = action.piece;

        // Check new state
        let (reward, done) = match self.final_reward(action.position) {
            Some(reward) => (reward, true),
            None if self.available_pieces.is_empty() => {
                // Finalize the game if the last piece is to be chosen
                let final_pos = self.available_positions[0];
                self.apply_position(final_pos);
                let reward = self.final_reward(final_pos).unwrap();
                (-reward, true)
            }
            None => (0., false),
        };

        (self.state, reward, done, self.actions())
    }
}

fn remove_item<T: PartialEq>(vec: &mut Vec<T>, item: &T) -> bool {
    vec.iter()
        .position(|el| el == item)
        .map(|i| vec.remove(i))
        .is_some()
}
