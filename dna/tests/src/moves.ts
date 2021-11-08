import { Orchestrator } from "@holochain/tryorama";
import { Scenario } from "@holochain/tryorama";
import { ScenarioApi } from "@holochain/tryorama/lib/api";
import { create } from "lodash";
import { config, installAgents, MEM_PROOF1, MEM_PROOF2} from "./install";
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
                {}, 
            ]);
        };
    )

async function makeMoves(
    gameHash: string, 
    alice, 
    bob,
    moves: Array<{x}>
) {
    
}