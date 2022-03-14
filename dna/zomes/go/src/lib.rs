use elo::{GoEloRating, GoGameInfo};
use hc_mixin_elo::*;
use hc_mixin_turn_based_game::*;
use hdk::prelude::holo_hash::{AgentPubKeyB64, EntryHashB64};
use hdk::prelude::*;

pub mod go_game;
pub mod elo;

use go_game::GoGame;

entry_defs![
    hc_mixin_elo::GameResult::entry_def(),
    GameMoveEntry::entry_def(),
    GameEntry::entry_def(),
    PathEntry::entry_def()
];

mixin_elo!(GoEloRating);
mixin_turn_based_game!(GoGame);

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {

    hc_mixin_elo::init_elo::<GoEloRating>()?;
    hc_mixin_turn_based_game::init_turn_based_games()
}

#[hdk_extern]
fn who_am_i(_: ()) -> ExternResult<AgentPubKeyB64>{
    Ok(agent_info()?.agent_latest_pubkey.into())
}

#[hdk_extern]
pub fn create_game(opponent: AgentPubKeyB64) -> ExternResult<EntryHashB64> {
    let my_pub_key = agent_info()?.agent_initial_pubkey;
    let players = vec![opponent.clone(), AgentPubKeyB64::from(my_pub_key.clone())];
    let game_hash = hc_mixin_turn_based_game::create_game(players.clone())?;
    

    Ok(game_hash)
}

#[derive (Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub struct PublishResultInput {
    last_game_move_hash: HeaderHashB64,
    game_hash: EntryHashB64,
    timestap: String,

    game_score: f32
}

#[hdk_extern]
pub fn publish_result(result: PublishResultInput) -> ExternResult<EntryHashB64>{
    let opponent = get_opponent_for_game(result.game_hash.clone())?;
   
    let _game_info = GoGameInfo {
        last_game_move_hash: result.last_game_move_hash.clone(),
        game_hash: result.game_hash,
    };
    let outcome = hc_mixin_elo::attempt_create_countersigned_game_result::<GoEloRating>(
        _game_info,
        opponent.clone(),
        result.game_score,
    )?;

    Ok(outcome)
}

#[hdk_extern]
fn get_opponent_for_game(game_hash: EntryHashB64) -> ExternResult<AgentPubKeyB64> {
    let game = get_game(game_hash)?;
    let my_pub_key = agent_info()?.agent_latest_pubkey;

    game.players
        .into_iter()
        .find(|p| !AgentPubKey::from(p.clone()).eq(&my_pub_key))
        .ok_or(WasmError::Guest(
            "I don't have any opponents in this game".into(),
        ))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CloseGameInput {
    game_hash: EntryHashB64,
    game_result_hash: EntryHashB64,
}

fn closing_game_result_tag() -> LinkTag {
    LinkTag::new("closing_game_result")
}

#[hdk_extern]
pub fn close_game(input: CloseGameInput) -> ExternResult<()> {
    hc_mixin_turn_based_game::remove_current_game(input.game_hash.clone().into())?;

    create_link(
        input.game_hash.into(),
        input.game_result_hash.into(),
        closing_game_result_tag(),
    )?;

    Ok(())
}



/* TODO: uncomment once validation rules of hc-turn-based-game are updated
#[hdk_extern]
fn validate_create_entry_game_entry(data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    hc_mixin_turn_based_game::prelude::validate_game_entry::<ChessGame, ChessGameMove>(data)
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