use goban::rules::game::Game;
use goban::rules::GobanSizes;
use goban::rules::CHINESE;
use hc_mixin_turn_based_game::{GameStatus, TurnBasedGame};
//use hdk::prelude::holo_hash::hash_b64::{AgentPubKeyB64, EntryHashB64};
use hdk::prelude::holo_hash::{AgentPubKeyB64, EntryHashB64};
use hdk::prelude::*;

use goban::rules::{Move, Player};
/* use goban::rules::*; */
/*
    This have:
        chain
            https://docs.rs/goban/0.17.0/goban/pieces/chain/struct.Chain.html
        goban
            https://docs.rs/goban/0.17.0/goban/pieces/goban/index.html
        stones
            https://docs.rs/goban/0.17.0/goban/pieces/stones/index.html
        territory
            https://docs.rs/goban/0.17.0/goban/pieces/territory/index.html
        util
            https://docs.rs/goban/0.17.0/goban/pieces/util/index.html
        zobrit
            https://docs.rs/goban/0.17.0/goban/pieces/zobrist/index.html
*/

pub enum GoGameResult {
    Draw,
    Winner(AgentPubKeyB64),
}

//Eliminar lo que diga Chess porque no va.
impl TurnBasedGame for GoGame {
    fn min_players() -> Option<usize> {
        Some(2)
    }

    fn max_players() -> Option<usize> {
        Some(2)
    }

    fn initial(players: Vec<AgentPubKeyB64>) -> Self {
        GoGame {
            white_address: players[0].clone().into(),
            black_address: players[1].clone().into(),
            all_moves: vec![],
        }
    }

    fn apply_move(self, game_move: GoGameMove, author: AgentPubKeyB64) -> ExternResult<GoGame> {
        let reference = self.clone();
        let mut game = reference.current_state().clone();
        
        match game_move {
            GoGameMove::PlacePiece { x, y } => {
                let go_to = Move::Play(x.clone(), y.clone());
                game.try_play(go_to)
                    .or(Err(WasmError::Guest("Error move.".into())))?;
            }
            GoGameMove::Resign => {
                let player = match self.white_address.eq(&author) {
                    true => Player::White,
                    false => Player::Black,
                };

                //Falta preguntar si el juego termino o no.
                let go_move: Move = Move::Resign(player);
                game.try_play(go_move)
                    .or(Err(WasmError::Guest("Error move.".into())))?;
            }
        }

        let new_game_state = self.with_new_move(game_move);

        Ok(new_game_state)
    }

    // Gets the winner for the game // remake this method
    fn status(&self) -> GameStatus {
        // Hacer un match para saber quien es el que gano, ya que actualmente retorna el jugador 0
        let reference = self.clone();
        match reference.current_state().outcome() {
            Some(_game) => GameStatus::Finished,

            None => GameStatus::Ongoing,
        }
    }

    type GameMove = GoGameMove;
}

#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub struct Piece {
  pub x: usize,
  pub y: usize,
}
 
#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub struct EntryGo {
  pub player_1: Vec<Piece>,
  pub player_2: Vec<Piece>,
} 

#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub struct GoGame {
    //Implementar la estructura del Go Game que se va a guardar en la zomes
    pub white_address: AgentPubKeyB64,
    pub black_address: AgentPubKeyB64,
    pub all_moves: Vec<GoGameMove>,
}

#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub struct TicTacToe {
    pub black_player: (AgentPubKey, Vec<Piece>),
    pub white_player: (AgentPubKey, Vec<Piece>),
    pub player_resigned: Option<AgentPubKeyB64>,
}

impl GoGame {
    pub fn current_state(self) -> Game {
        const SIZE: GobanSizes = GobanSizes::Custom(16, 16);

        let mut game = Game::new(SIZE, CHINESE);

        let mut active_player = Player::White;

        for go_game_move in self.all_moves {
            let go_move = go_game_move.into_go_move(active_player);

            game.play(go_move);

            active_player = match active_player {
                Player::Black => Player::White,
                Player::White => Player::Black,
            };
        }
        game
    }

    fn with_new_move(self, go_game_move: GoGameMove) -> Self {
        let mut all_moves = self.all_moves.clone();
        all_moves.push(go_game_move);
        GoGame {
            white_address: self.white_address,
            black_address: self.black_address,
            all_moves,
        }
    }
}

#[derive(Clone, SerializedBytes, Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
pub enum GoGameMove {
    PlacePiece { x: u8, y: u8 },
    Resign,
}

impl GoGameMove {
    fn into_go_move(self, player: Player) -> Move {
        match self {
            GoGameMove::PlacePiece { x, y } => Move::Play(x.clone(), y.clone()),
            GoGameMove::Resign => Move::Resign(player),
        }
    }
}

#[derive(Clone, Debug)]
pub enum GoGameScore {
    WinnerByScore(Player, f32),
    WinnerByResign(Player),
    WinnerByTime(Player),
    WinnerByForfeit(Player),
    Draw,
}

#[derive(Clone, SerializedBytes, Deserialize, Serialize, Debug)]
pub struct MakeMoveInput {
    pub game_hash: EntryHashB64,
    pub previous_move_hash: Option<EntryHashB64>,
    pub game_move: GoGameMove,
}