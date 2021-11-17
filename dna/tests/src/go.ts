import { ScenarioApi } from "@holochain/tryorama/lib/api";
import { Orchestrator } from "@holochain/tryorama";
import Base64 from "js-base64";
import {
  installAgents,
  MEM_PROOF1,
  MEM_PROOF2,
  MEM_PROOF_READ_ONLY,
} from "./install";

import { createGame, delay, getCurrentGames, getGameResultsForAgents, getMyGameResults, makeMove, serializeHash} from "./utils";
import { MakeMoveInput } from "./types";

export default function (config) {
  let orchestrator = new Orchestrator();

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

      await delay(3000);
      const new_game_address: string = await createGame(bobbyPubKey)(
        alice_conductor
      );
      await delay(3000);

      console.log("the result is this:");
      console.log(new_game_address);
      /*Cambiar desde este punto los inserte al tableroI   */

      const movement_input: MakeMoveInput = {
        game_hash: new_game_address,
        previous_move_hash: null,
        game_move: { type: "PlacePiece", x: "2", y: "4" },
      };

      const make_move = await makeMove(movement_input)(bobby_conductor);
      await delay(1000);

      const links = await alice_conductor.call(
        "holoGo",
        "get_game_moves",
        new_game_address
      );

      t.equal(links.length, 1);

      const aliceCurrentGames = await getCurrentGames()(alice_conductor);
      t.equal(Object.keys(aliceCurrentGames).length, 1);
      const aliceGamesResults = await getGameResultsForAgents(bobby_conductor)([
        [alicePubKey]
      ]);
      t.equal(aliceGamesResults.length, 0);

      const bobCurrentGames = await getCurrentGames()(bobby_conductor);
      t.equal(bobCurrentGames.length, 1);
      const bobGamesResults = await getMyGameResults(bobby_conductor)([
        [bobbyPubKey]
      ]);
      t.equal(bobGamesResults.length, 0);

      const resign_move: MakeMoveInput = {
        game_hash: new_game_address,
        previous_move_hash: make_move,
        game_move: { type: "Resign" },
      };
      await makeMove(resign_move)(alice_conductor);
      await alice_conductor.call("holoGo", "publish_result", {
        game_hash: new_game_address,
        timestamp: Date.now(),
        white_player: bobbyPubKey,
        black_player: alicePubKey,
        winner: {
          White: null,
        },
        num_of_moves: 1,
      });
      await delay(1000);

      const aliceCurrentGames1 = await getCurrentGames()(alice_conductor);
      t.equal(aliceCurrentGames1.length, 0);
      const aliceGamesResults1 = await getMyGameResults(alice_conductor)(
        [alicePubKey]
      );
      t.equal(aliceGamesResults1.length, 1);

      const bobCurrentGames1 = await getCurrentGames()(bobby_conductor);
      t.equal(bobCurrentGames1.length, 0);
      const bobGamesResults1 = await getMyGameResults(bobby_conductor)(
        [bobbyPubKey]
      );
      t.equal(bobGamesResults1.length, 1);
    }
  );

  orchestrator.run();
}
