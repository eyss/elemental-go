import { Orchestrator } from "@holochain/tryorama";
import { ScenarioApi } from "@holochain/tryorama/lib/api";
import { config, installAgents, MEM_PROOF1, MEM_PROOF2 } from "./install";
import { MakeMoveInput } from "./types";
import { createGame, delay, makeMove, serializeHash } from "./utils";

export default (orchestrator: Orchestrator<any>) => 
    orchestrator.registerScenario(
        "go zome test", 
        async (s: ScenarioApi, t) => {
          
            const [conductor] = await s.players([config]);

            conductor.setSignalHandler((signal) => {
                console.log("Players has received Signal: ", signal);
            });
            
            const [alice_happ, bobby_happ] = await installAgents(
                conductor, 
                ["alice", "bob"],
                [MEM_PROOF1, MEM_PROOF2]
            );
            
            const alicePubKey = serializeHash(alice_happ.agent);
            const bobbyPubKey = serializeHash(bobby_happ.agent);

            const alice = alice_happ.cells[0];
            const bob = bobby_happ.cells[0];
            
            await delay(3000);
            const new_game_address: string = await createGame(alicePubKey)(bob);
            await delay(4000);

            console.log("the result is this:");
            console.log(new_game_address);

            await makeMoves(new_game_address, alice, bob, [            
                {x: 0, y: 3}, 
                {x: 2, y: 5}, 
                {x: 2, y: 2}, 
                {x: 2, y: 4}, 
                {x: 3, y: 3}, 
                {x: 3, y: 4}, 
            ]);
        }
    );

async function makeMoves(
    gameHash: string,
    alice,
    bobby,
    moves: Array<{ x: number; y: number }>
  ) {
    let previous_move_hash = null;
    let aliceTurn = true;
    for (const move of moves) {
      const movement_input: MakeMoveInput = {
        game_hash: gameHash,
        previous_move_hash: previous_move_hash,
        game_move: { type: "PlacePiece", x: move.x, y: move.y },
        timestap: "",
        myScore: 0
      };
      console.log("making move: ", movement_input);
      try {
        previous_move_hash = await makeMove(movement_input)(
          aliceTurn ? alice : bobby
        );
      } catch (e) {
        if (
          JSON.stringify(e).includes(
            "Cannot make move: can't fetch the previous move hash yet"
          )
        ) {
          await delay(4000);
  
          previous_move_hash = await makeMove(movement_input)(
            aliceTurn ? alice : bobby
          );
        } else throw e;
      }
      await delay(4000);
      aliceTurn = !aliceTurn;
    }
  }