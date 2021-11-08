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
                console.log("Players has received Signal: ", signal.data.playload.playload);
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
//              letra       numero                
                {x: " ", y: " "}, 
                {x: " ", y: " "}, 
                {x: " ", y: " "}, 
                {x: " ", y: " "}, 
                {x: " ", y: " "}, 
                {x: " ", y: " "}, 
            ]);
        }
    );

    async function makeMoves(
        gameHash: string,
        alice,
        bobby,
        moves: Array<{ x: string; y: string }>
      ) {
        let previous_move_hash = null;
        let aliceTurn = true;
        for (const move of moves) {
          const movement_input: MakeMoveInput = {
            game_hash: gameHash,
            previous_move_hash,
            game_move: { type: "PlacePiece", from: move.x, to: move.y },
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
              await delay(2000);
      
              previous_move_hash = await makeMove(movement_input)(
                aliceTurn ? alice : bobby
              );
            } else throw e;
          }
          await delay(2000);
          aliceTurn = !aliceTurn;
        }
      }