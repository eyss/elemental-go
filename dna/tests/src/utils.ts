/* import { Conductor } from "@holochain/tryorama/lib/conductor";
import { Agent } from "http"; */
import { Base64 } from "js-base64";
import {MakeMoveInput} from "./types";

export const delay = (ms) => new Promise((r) => setTimeout(r, ms));
export const createGame = (opponent: string) => (conductor) => 
    conductor.call("go", "create_game", opponent);
export const makeMove = (make_move_input: MakeMoveInput) => (conductor) =>
    conductor.call("go", "make_move", make_move_input);
export const getCurrentGames = () => (conductor) => (agents) =>
    conductor.call("go", "get_currently_games", null);
export const getMyGameResults = (conductor) => (agents) =>
    conductor.call("go", "get_game_result_for_agents", agents);

export function serializeHash(hash){
    return `u${Base64.fromUint8Array(hash, true)}`;
}
