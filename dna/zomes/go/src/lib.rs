use hdk::prelude::holo_hash::{AgentPubKeyB64, EntryHashB64 };
use hc_mixin_elo::*;
use hc_mixin_turn_based_game::*;
use hdk::prelude::*;

pub mod go_game;
pub mod elo;

use go_game::GoGame;

entry_defs![
    GameMoveEntry::entry_def(),
    GameEntry::entry_def(),
    GoGameResult::entry_def()
];

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    hc_mixin_turn_based_game::init_turn_based_games()
}

#[hdk_extern]
pub fn create_game(opponent: AgentPubKeyB64) -> ExternResult<EntryHashB64> {
    //let hash = hc_mixin_turn_based_game::create_game(opponent, players);
    let my_pub_key = agent_info()?.agent_initial_pubkey;
    let players = vec![opponent.clone(), AgentPubKeyB64::from(my_pub_key.clone())];
    let game_hash = hc_mixin_turn_based_game::create_game(players.clone())?;
    
    Ok(game_hash)

}


#[hdk_extern]
pub fn get_game(game_hash: EntryHashB64) -> ExternResult<GameEntry> {
    hc_mixin_turn_based_game::get_game(game_hash)
    
}

#[hdk_extern]
pub fn get_game_moves(game_hash: EntryHashB64) -> ExternResult<Vec<MoveInfo>> {
    hc_mixin_turn_based_game::get_game_moves(game_hash)
    
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
