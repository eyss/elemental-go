use hdk::prelude::holo_hash::{AgentPubKeyB64, EntryHashB64};
use hdk::prelude::*;
use hc_turn_based_game::prelude::*;

//use goban::rules::*;
//use goban::rules::game::*;



pub mod go_game;
pub mod go_game_result;
pub mod current_games;

use go_game::{MakeMoveInput};

use go_game_result::GoGameResult;


entry_defs![
    GameMoveEntry::entry_def(),
    GameEntry::entry_def(),
    GoGameResult::entry_def()
];

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    hc_turn_based_game::prelude::init_turn_based_games()
}

#[hdk_extern]
pub fn create_game(opponent: AgentPubKeyB64) -> ExternResult<EntryHashB64> {
    let my_pub_key = agent_info()?.agent_initial_pubkey;
    let players = vec![opponent.clone(), AgentPubKeyB64::from(my_pub_key.clone())];

    let game_hash = hc_turn_based_game::prelude::create_game(players.clone())?;



    current_games::add_current_game(
        game_hash.clone().into(),
        players.into_iter().map(|p| p.into()).collect(),
    )?;

    Ok(game_hash)
}

#[hdk_extern]
pub fn make_move(input: MakeMoveInput) -> ExternResult<EntryHashB64> {
    hc_turn_based_game::prelude::create_move(
        input.game_hash,
        input.previous_move_hash,
        input.game_move,
    )
}

#[hdk_extern]
pub fn get_game(game_hash: EntryHashB64) -> ExternResult<GameEntry> {
    hc_turn_based_game::prelude::get_game(game_hash)
}

#[hdk_extern]
pub fn get_game_moves(game_hash: EntryHashB64) -> ExternResult<Vec<MoveInfo>> {
    hc_turn_based_game::prelude::get_game_moves(game_hash)
}

#[hdk_extern]
pub fn publish_result(result: GoGameResult) -> ExternResult<()> {
    go_game_result::publish_result(result.clone())?;

    let players: Vec<AgentPubKey> = vec![result.black_player.into(), result.white_player.into()];

    current_games::remove_current_game(result.game_hash.into(), players)?;

    Ok(())
}

#[hdk_extern]
pub fn get_my_game_results(_: ()) -> ExternResult<Vec<(EntryHashB64, GoGameResult)>> {
    go_game_result::get_my_game_results()
}

#[hdk_extern]
pub fn get_my_current_games(_: ()) -> ExternResult<Vec<EntryHashB64>> {
    current_games::get_my_current_games()
}

/* TODO: uncomment once validation rules of hc-turn-based-game are updated
#[hdk_extern]
fn validate_create_entry_game_entry(data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    hc_turn_based_game::prelude::validate_game_entry::<ChessGame, ChessGameMove>(data)
    // TODO: add validation for read-only agents
}

#[hdk_extern]
fn validate_create_entry_game_move_entry(
    data: ValidateData,
) -> ExternResult<ValidateCallbackResult> {
    hc_turn_based_game::prelude::validate_game_move_entry::<ChessGame, ChessGameMove>(data)
    // TODO: add validation for read-only agents
}
 */