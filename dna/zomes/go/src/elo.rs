use hc_mixin_elo::*;
use hc_mixin_turn_based_game::{GameMoveEntry, TurnBasedGame, GameStatus};
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::go_game::{GoGame/* , GoGameResult */};

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub struct GoGameInfo {
    pub last_game_move_hash: HeaderHashB64,
    pub game_hash: EntryHashB64
}

pub struct GoEloRating;

impl EloRatingSystem for GoEloRating {
    type GameInfo = GoGameInfo;

    fn validate_game_result(
        game_info: GoGameInfo,
        _game_result_info: GameResultInfo,
    ) -> ExternResult<ValidateCallbackResult> {
        let last_move_element = must_get_valid_element(game_info.last_game_move_hash.into())?;

        let maybe_move: Option<GameMoveEntry> = last_move_element.entry().to_app_option()?;

        if let Some(game_move) = maybe_move {
            let go_game = GoGame::try_from(game_move.resulting_game_state)
                .or(Err(WasmError::Guest("Malformed game state".into())))?;
            let result = go_game.status();
            
            match result {
                GameStatus::Finished => {
                    return Ok(
                        ValidateCallbackResult::Invalid("Game finished".into()),   
                    );
                },
                GameStatus::Ongoing => {
                    return Ok(
                        ValidateCallbackResult::Invalid("Game ongoing".into()),
                    );
                },
            }

        } else {
            let entry_hash = last_move_element
                .header()
                .entry_hash()
                .ok_or(WasmError::Guest(
                    "Bad game_result_element: no entry hash".into(),
                ))?;
            return Ok(ValidateCallbackResult::UnresolvedDependencies(vec![
                entry_hash.clone().into(),
            ]));
        }
    }
}