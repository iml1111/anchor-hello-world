use crate::errors::TicTacToeError;
use anchor_lang::prelude::*;
use num_derive::*;
use num_traits::*;

#[account]
pub struct Game {
	// 게임을 진행하는 2명의 플레이어
	players: [Pubkey; 2], // (32 * 2)
	// 현재 게임이 몇 번째 턴인가 (0: 시작 안함)
	turn: u8, // 1
	// Game을 진행하는 9 x 9 보드판
	board: [[Option<Sign>; 3]; 3], // 9 * (1 + 1) = 18
	// Game의 상태 여부 or 승리자
	state: GameState, // 32 + 1
}

impl Game {
	// Game Struct total size
	pub const MAXIMUM_SIZE: usize = (32 * 2) + 1 + (9 * (1 + 1)) + (32 + 1);

	pub fn start(&mut self, players: [Pubkey; 2]) -> Result<()> {
		require_eq!(self.turn, 0, TicTacToeError::GameAlreadyStarted);
		self.players = players;
		self.turn = 1;
		Ok(())
	}

	pub fn is_active(&self) -> bool {
		// game의 활성화 유무 판단
		self.state == GameState::Active
	}

	fn current_player_index(&self) -> usize {
		// 짝수/홀수 판별을 통해 어떤 유저 차례인지 판별
		((self.turn - 1) % 2) as usize
	}

	pub fn current_player(&self) -> Pubkey {
		// 현재 player를 호출함
		self.players[self.current_player_index()]
	}

	pub fn play(&mut self, tile: &Tile) -> Result<()> {
		// tile을 입력받아, board에 tile을 배치함.
		require!(self.is_active(), TicTacToeError::GameAlreadyOver);

		match tile {
			tile @ Tile {
				row: 0..=2,
				column: 0..=2,
			} => match self.board[tile.row as usize][tile.column as usize] {
				Some(_) => return Err(TicTacToeError::TileAlreadySet.into()),
				None => {
					self.board[tile.row as usize][tile.column as usize] = Some(Sign::from_usize(self.current_player_index()).unwrap());
				}
			},
			_ => return Err(TicTacToeError::TileOutOfBounds.into()),
		}

		self.update_state();

		if GameState::Active == self.state {
			self.turn += 1;
		}
		Ok(())
	}

	fn is_winning_trio(&self, trio: [(usize, usize); 3]) -> bool {
		// 입력받은 3개의 tile이 같은 tile인지 확인
		let [first, second, third] = trio;
		self.board[first.0][first.1].is_some()
			&& self.board[first.0][first.1] == self.board[second.0][second.1]
			&& self.board[first.0][first.1] == self.board[third.0][third.1]
	}

	fn update_state(&mut self) {
		for i in 0..=2 {
			if self.is_winning_trio([(i, 0), (i, 1), (i, 2)]) {
				self.state = GameState::Win {
					winner: self.current_player(),
				};
				return;
			}
			if self.is_winning_trio([(0, i), (1, i), (2, i)]) {
				self.state = GameState::Win {
					winner: self.current_player(),
				};
				return;
			}
		}

		if self.is_winning_trio([(0,0), (1,1), (2,2)]) || self.is_winning_trio([(0,2), (1,1), (2,0)]) {
			self.state = GameState::Win {
				winner: self.current_player(),
			};
			return;
		}

		
		for row in 0..=2 {
            for column in 0..=2 {
                if self.board[row][column].is_none() {
                    return;
                }
            }
        }
        // 모든 Tile이 채워졌음에도 trio가 되지 않으면 무승부로 종료.
        self.state = GameState::Draw;
	}
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
	Active, 
	Draw, 
	Win { winner: Pubkey },
}

#[derive(
	AnchorSerialize,
	AnchorDeserialize,
	FromPrimitive,
	ToPrimitive,
	Copy,
	Clone,
	PartialEq,
	Eq
)]
pub enum Sign {
	X,
	O
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Tile {
    row: u8,
    column: u8,
}