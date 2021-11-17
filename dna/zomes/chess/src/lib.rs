use hdk::prelude::holo_hash::{AgentPubKeyB64, EntryHashB64};
use hdk::prelude::*;
use hc_mixin_turn_based_game::*;
use hc_mixin_elo::*;
//use go_game::{MakeMoveInput};
use go_game_result::GoGameResult;
use elo::{GoEloRating, GoGameInfo};

pub mod go_game;
pub mod elo;

use go_game::GoGame;

pub mod go_game_result;
pub mod current_games;


use holo_hash::HeaderHashB64;

entry_defs![
    GameMoveEntry::entry_def(),
    GameEntry::entry_def(),
    GoGameResult::entry_def()
];


mixin_elo!(GoEloRating);
mixin_turn_based_game!(GoGame);


#[hdk_extern]
fn who_am_i(_: ()) -> ExternResult<AgentPubKeyB64>{
    Ok(agent_info()?.agent_latest_pubkey.into())
}

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    hc_mixin_turn_based_game::init_turn_based_games()
}

#[hdk_extern]
pub fn create_game(opponent: AgentPubKeyB64) -> ExternResult<EntryHashB64> {
    let my_pub_key = agent_info()?.agent_initial_pubkey;
    let players = vec![opponent.clone(), AgentPubKeyB64::from(my_pub_key.clone())];

    let game_hash = hc_mixin_turn_based_game::create_game(players.clone())?;
    let player_aux = players.clone();

    current_games::add_current_game(
        game_hash.clone().into(),
        player_aux.into_iter().map(|p| p.into()).collect(),
    )?;

    Ok(game_hash)
}

/* #[hdk_entry]
 */
pub fn make_move(make_move_input: MakeMoveInput)->  ExternResult<GoGame>{
    let input_is = make_move_input.clone();
    let game = mixin_turn_based_game(input_is);
    //Preguntarle a Guillem como alcanzo a la funcion makemove de mixin_turn based game
    //Revisar el chess
    Ok(game)
} 



#[derive (Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub struct PublishResultInput {
    game_hash: EntryHashB64,
    last_game_move_hash: HeaderHashB64,
    game_score: f32
}

#[hdk_extern]
pub fn publish_result(result: PublishResultInput) -> ExternResult<CreateGameResultOutcome>{
    let _game_info = GoGameInfo {
        last_game_move_hash: result.last_game_move_hash.clone(),
    };
    let opponent = get_opponent_for_game(result.game_hash.clone())?;
    
    let _game_info = GoGameInfo {
        last_game_move_hash: result.last_game_move_hash.clone(),
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

#[hdk_extern]
pub fn get_game_go(game_hash: EntryHashB64) -> ExternResult<GameEntry> {
    hc_mixin_turn_based_game::get_game(game_hash)
}

#[hdk_extern]
pub fn get_game_moves_go(game_hash: EntryHashB64) -> ExternResult<Vec<MoveInfo>> {
    hc_mixin_turn_based_game::get_game_moves(game_hash)
}

#[hdk_extern]
pub fn publish_result_go(result: GoGameResult) -> ExternResult<()> {
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
pub fn get_my_current_games_go(_: ()) -> ExternResult<Vec<EntryHashB64>> {
    current_games::get_my_current_games()
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