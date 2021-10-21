use hdk::prelude::holo_hash::{AgentPubKeyB64, EntryHashB64};
use hdk::prelude::*;
use hc_mixin_turn_based_game::{GameStatus, TurnBasedGame};
use goban::rules::{GobanSizes};
use goban::rules::game::Game;
use goban::rules::CHINESE;

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
    pub white_address : AgentPubKeyB64,
    pub black_address : AgentPubKeyB64,
    pub game: Game,    
}


#[derive(Clone, SerializedBytes, Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
pub enum GoGameMove {
    PlacePiece {
        x: u8,
        y: u8
    },
    Resign,
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
    
    fn initial(players: &Vec<AgentPubKeyB64>) -> Self {
        const SIZE: GobanSizes = GobanSizes::Custom(16, 16);
        GoGame {
            white_address: players[0].clone().into(),
            black_address: players[1].clone().into(),
            game: Game::new( SIZE,  CHINESE),
        }
    }

    fn apply_move(
        &mut self,
        game_move: &GoGameMove,
        author: AgentPubKeyB64,
    
    ) -> ExternResult<()> {
        match game_move{
            GoGameMove::PlacePiece {
                x, 
                y
            } =>{
                let go_to = Move::Play(x.clone(), y.clone());
                self.game.try_play(go_to)
                    .or(Err(WasmError::Guest("Error move.".into())))?;
            }
            GoGameMove::Resign=>{
                
                let player = match self.white_address.eq(&author){
                    true => Player::White,
                    false => Player::Black
                };

                //Falta preguntar si el juego termino o no.
                let go_move: Move = Move::Resign(player);
                self.game.try_play(go_move)
                    .or(Err(WasmError::Guest("Error move.".into())))?;

            }
        }
        return Ok(());
    }
    // Gets the winner for the game // remake this method
    fn status(&self) -> GameStatus{

        //Hacer un match para saber quien es el que gano, ya que actualmente retorna el jugador 0
                 
        match self.game.outcome(){
            Some(game)=>GameStatus::Finished, 
    
            None=>GameStatus::Ongoing,
        }
    }

    type GameMove=GoGameMove;
}

#[derive(Clone, SerializedBytes, Deserialize, Serialize, Debug)]
pub struct MakeMoveInput {
    pub game_hash: EntryHashB64,
    pub previous_move_hash: Option<EntryHashB64>,
    pub game_move: GoGameMove,
}
