import { ScenarioApi } from "@holochain/tryorama/lib/api";
import { Orchestrator } from "@holochain/tryorama";
import Base64 from "js-base64";
import {
  installAgents,
  MEM_PROOF1,
  MEM_PROOF2,
  MEM_PROOF_READ_ONLY,
  config
} from "./install";

import { createGame, delay, getCurrentGames, getGameResultsForAgents, getMyGameResults, makeMove, serializeHash} from "./utils";
import { MakeMoveInput } from "./types";

export default function (orchestrator: Orchestrator<any>) {
  
  orchestrator.registerScenario(
    "go zome tests",
    async (s: ScenarioApi, t) => {
      const [conductor] = await s.players([config]);

      conductor.setSignalHandler((signal) => {
        console.log("Player has received Signal:", signal.data.payload.payload);
      });

      const [alice_happ, bobby_happ] = await installAgents(
        conductor,
        ["alice", "bob"],
        [MEM_PROOF1, MEM_PROOF2]
      );

      const alicePubKey = serializeHash(alice_happ.agent);
      const bobbyPubKey = serializeHash(bobby_happ.agent);

      const alice_conductor = alice_happ.cells[0];
      const bobby_conductor = bobby_happ.cells[0];
      await bobby_conductor.call("profiles", "get_my_profile", null);

      await delay(12000);
      const new_game_address: string = await createGame(bobbyPubKey)(
        alice_conductor
      );
      await delay(12000);

      console.log("the result is this:"); 
      console.log(new_game_address);
      /*Cambiar desde este punto los inserte al tableroI   */
      
      console.log("-------------------------------------------------------------------------------------------------------------->First Test  Make Move<------------------------------");
      let dateTime = new Date().toString();
      const movement_input: MakeMoveInput = {
        game_hash: new_game_address,
        previous_move_hash: null,
        game_move: { type: "PlacePiece", x: 2, y: 4 },
        timestap: dateTime, 
        myScore: 0
      };

      let lastMoveHash = await makeMove(movement_input)(bobby_conductor);
      await delay(12000);

      const links = await alice_conductor.call(
        "go",
        "get_game_moves",
        new_game_address
      );

      t.equal(links.length, 1);

      console.log("-------------------------------------------------------------------------------------------------------------->Second Test Get Current Game<------------------------------");
      const aliceCurrentGames = await getCurrentGames()(alice_conductor);
      console.log("aliceCurrentGames ",aliceCurrentGames);
      t.equal(Object.keys(aliceCurrentGames).length, 1);

      console.log("------------------------------------------------------------------------------------------------------------->Thirth Test Get Game Result for Agents<------------------------------");
      const aliceGamesResults = await getGameResultsForAgents(bobby_conductor)([
        alicePubKey
      ]);
      t.equal(Object.keys(aliceGamesResults[alicePubKey]).length, 0);

      console.log("-------------------------------------------------------------------------------------------------------------->Fourth Test Get Current Games<------------------------------");
      const bobCurrentGames = await getCurrentGames()(bobby_conductor);
      t.equal(Object.keys(bobCurrentGames).length, 1);
      const bobGamesResults = await getMyGameResults(bobby_conductor)(
        [bobbyPubKey]
      );
      t.equal(Object.keys(bobGamesResults).length, 1);
      console.log("Return bob: ", bobGamesResults);
      console.log("-------------------------------------------------------------------------------------------------------------->Fiveth Test Publish Result<------------------------------");
      const resign_move: MakeMoveInput = {
        game_hash: new_game_address,
        previous_move_hash: lastMoveHash,
        game_move: { type: "Resign" },
        timestap: dateTime,
        myScore: 0,
      };
      
      console.log("Try publishing Resign\n");
      lastMoveHash = await makeMove(resign_move)(alice_conductor);
      await delay(4000);

      const outcome = await alice_conductor.call("go", "publish_result", {
        game_hash: new_game_address,
        last_game_move_hash: lastMoveHash,
        timestap: dateTime,
        game_score: 0,
        //Now require solve to input score Game
      });
      t.equal(outcome.type, "Published");

      const game_result_hash = outcome.game_result_hash;

      await delay(10000);
      
      await alice_conductor.call("go", "link_my_game_results", [
        game_result_hash,
      ]);

      /*
      await alice_conductor.call("chess", "link_my_game_results", [
        game_result_hash,
      ]);
      await alice_conductor.call("chess", "close_game", {
        game_hash: new_game_address,
        game_result_hash,
      });
      */
      await alice_conductor.call("go", "close_game", {
        game_hash: new_game_address,
        game_result_hash,
      });



      const aliceCurrentGames1 = await getCurrentGames()(alice_conductor);

      console.log("Alice Currente Game ", aliceCurrentGames1);

      t.equal(Object.keys(aliceCurrentGames1).length, 0);

      console.log("-------------------------------------------------------------------------------------------------------------->Sixth Test<------------------------------");
      const aliceGamesResults1 = await getMyGameResults(alice_conductor)(
        [alicePubKey]
      );
      t.equal(Object.keys(aliceGamesResults1).length, 1);

      console.log("-------------------------------------------------------------------------------------------------------------->Seventh Test<------------------------------");
      const bobCurrentGames1 = await getCurrentGames()(bobby_conductor);
      t.equal(Object.keys(bobCurrentGames1).length, 0);
      console.log("-------------------------------------------------------------------------------------------------------------->Eighth Test<------------------------------");
      const bobGamesResults1 = await getMyGameResults(bobby_conductor)(
        [bobbyPubKey]
      );
      t.equal(Object.keys(bobGamesResults1).length, 1);
    }
  );

  
}
