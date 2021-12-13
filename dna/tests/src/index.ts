/* import { Config, NetworkType, TransportConfigType } from "@holochain/tryorama";
 */

import {
  Config, 
  NetworkType,
  Orchestrator,
  TransportConfigType,
} from '@holochain/tryorama';


// QUIC
/* const network = {
  network_type: NetworkType.QuicBootstrap,
  transport_pool: [{ type: TransportConfigType.Quic }],
  bootstrap_service: "https://bootstrap-staging.holo.host/",
};
 */

 
import go from "./go";
import moves from "./moves";

let orchestrator = new Orchestrator();
go(orchestrator);
orchestrator.run();


orchestrator = new Orchestrator();
moves(orchestrator);
orchestrator.run(); 
