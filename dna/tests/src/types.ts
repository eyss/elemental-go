import { InstallAgentsHapps } from "@holochain/tryorama";

/* export interface Installables {
  [key: string]: InstallAgentsHapps;
} */


export interface Installables {
  [key: string]: InstallAgentsHapps;
}

export type MakeMoveInput = {
  game_hash: string,
  previus_move_hash: string | null,
  game_move: any,
  x: number,
  y: number,
}