use hc_mixin_elo::*;
use hc_mixin_turn_based_game::GameMoveEntry;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::go_game::{GoGame};
use crate::go_game;
/* use crate::go_game::*; */


#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub struct GoGameInfo {
    pub last_game_move_hash: HeaderHashB64,
}

pub struct GoEloRating;

impl EloRatingSystem for GoEloRating {
    type GameInfo = GoGameInfo;

    fn validate_game_result(
        game_info: GoGameInfo,
        game_result_info: GameResultInfo,
    ) -> ExternResult<ValidateCallbackResult> {
        let last_move_element = must_get_valid_element(game_info.last_game_move_hash.into())?;

        let maybe_move: Option<GameMoveEntry> = last_move_element.entry().to_app_option()?;

        if let Some(game_move) = maybe_move {
            let go_game = GoGame::try_from(game_move.resulting_game_state)
                .or(Err(WasmError::Guest("Malformed game state".to_string()).into()))?;
            let result = go_game::get_result()?;

            match result {
                None => Ok(ValidateCallbackResult::Invalid(
                    "Game has not finished yet".into(),
                )),
                Some(go_result) => match go_result {
                    GoGameResult::Draw if game_result_info.score_player_a != 0.5 => {
                        Ok(ValidateCallbackResult::Invalid("".into()))
                    }
                    GoGameResult::Winner(winner_pub_key)
                        if game_result_info.player_a.eq(&winner_pub_key)
                            && game_result_info.score_player_a != 1.0 =>
                    {
                        Ok(ValidateCallbackResult::Invalid(
                            "Winner does not have 1.0 as score".into(),
                        ))
                    }
                    GoGameResult::Winner(winner_pub_key)
                        if game_result_info.player_b.eq(&winner_pub_key)
                            && game_result_info.score_player_a != 0.0 =>
                    {
                        Ok(ValidateCallbackResult::Invalid(
                            "Loser does not have 0.0 as score".into(),
                        ))
                    }
                    _ => Ok(ValidateCallbackResult::Valid),
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
