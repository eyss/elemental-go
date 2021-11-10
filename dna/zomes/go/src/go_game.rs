use goban::rules::game::Game;
use goban::rules::GobanSizes;
use goban::rules::CHINESE;
use hc_mixin_turn_based_game::{GameStatus};
use hdk::prelude::holo_hash::{AgentPubKeyB64, EntryHashB64};
use hdk::prelude::*;
use hdk::prelude::holo_hash::*;
//use hc_mixin_turn_based_game::TurnBasedGame::*; 
//use hc_mixin_turn_based_game::TurnBasedGame;
use hc_mixin_turn_based_game::*;

use goban::rules::Move;
use goban::rules::Player;


/*
    Esto trae:
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
#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub struct GoGame {
    //Implementar la estructura del Go Game que se va a guardar en la zomes
    pub white_address: AgentPubKeyB64,
    pub black_address: AgentPubKeyB64,
    pub resigned_player: Option<AgentPubKeyB64>,
    pub all_moves: Vec<GoGameMove>,
    pub board_state: String,
}

pub enum GoGameResult{
    Draw, 
    Winner(AgentPubKeyB64),
}

impl GoGame {
    
    pub fn game_state(&self) -> ExternResult<Game> {
//      Game::from_str(self.board_state.as_str())
        
        Game::from_str(self.board_state.as_str())
            .or(Err(WasmError::Gues("Invalid board State")))
    }

    pub fn get_result(&self) -> ExternResult<Option<GoGameResult>> {
        let game = self.game_state()?;

        if let Some(player) = self.resigned_player.clone() {
            return match self.white_address.eq(&player){
                true => Ok(Some(GoGameResult::Winner(self.black_address.clone()))),
                false => Ok(Some(GoGameResult::Winner(self.white_address.clone()))),
            }
        }
        match game.resume() {
            None => Ok(None),
            Some(result) => match result{
                GoGameResult::Draw=>{
                    Ok(Some(GoGameResult::Draw))
                }
                GoGameResult::Winner()
        }
    }

}

#[derive(Clone, SerializedBytes, Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
pub enum GoGameMove {
    PlacePiece { 
        x: u8, 
        y: u8, 
    },
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
            resigned_player: None,
            board_state: Game::new(size, rule).to_string(),
            all_moves: vec![],
        }
    }

    fn apply_move(self, game_move: GoGameMove, author: AgentPubKeyB64) -> ExternResult<GoGame> {
        let mut game = self.clone().current_state();

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
        if let Some(_) = self.clone().resigned_player{
            return GameStatus::Finished;
        }


        match self.clone().current_state().outcome() {
            Some(_game) => GameStatus::Finished,

            None => GameStatus::Ongoing,
        }
    }

    type GameMove = GoGameMove;
}

#[derive(Clone, SerializedBytes, Deserialize, Serialize, Debug)]
pub struct MakeMoveInput {
    pub game_hash: EntryHashB64,
    pub previous_move_hash: Option<HeaderHashB64>,
    pub game_move: GoGameMove,
}

