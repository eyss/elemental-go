use hdk::prelude::holo_hash::{AgentPubKeyB64, EntryHashB64};
use hdk::prelude::*;
use hc_turn_based_game::prelude::TurnBasedGame;
use goban::rules::{GobanSizes};
use goban::rules::game::Game;
use goban::rules::CHINESE;

use goban::rules::Move;
use goban::rules::EndGame;
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
#[derive(Clone, Debug)]
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
impl TurnBasedGame<GoGameMove> for GoGame {
    
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
        _players: &Vec<AgentPubKeyB64>,
        author_index: usize,
    
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
                let black_player: Player = self::Player::Black;
                let white_player: Player = self::Player::White;

                //Falta preguntar si el juego termino o no.
                
                if _players[author_index] == self.white_address{
                    Move::Resign(white_player);    
                }else{
                    Move::Resign(black_player);
                }
            }
        }
        return Ok(());
    }

    // Gets the winner for the game // remake this method
    fn get_winner(&self, players: &Vec<AgentPubKeyB64>) -> Option<AgentPubKeyB64>{

        //Hacer un match para saber quien es el que gano, ya que actualmente retorna el jugador 0
                 
        match self.game.outcome(){
            Some(outcome)=> match outcome{
                EndGame::WinnerByScore (_, _) => Some(players[0].clone()),
                EndGame::WinnerByResign(_player) => Some(players[0].clone()),// Revisar con Guillem 
                EndGame::WinnerByTime(_) => Some(players[0].clone()),
                EndGame::WinnerByForfeit(_) => Some(players[0].clone()),
                EndGame::Draw => Some(players[0].clone()),
            },
            None => None,


        }
         

    }
}

#[derive(Clone, SerializedBytes, Deserialize, Serialize, Debug)]
pub struct MakeMoveInput {
    pub game_hash: EntryHashB64,
    pub previous_move_hash: Option<EntryHashB64>,
    pub game_move: GoGameMove,
}
