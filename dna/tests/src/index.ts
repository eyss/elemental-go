import { Config, NetworkType, TransportConfigType } from "@holochain/tryorama";

import holoGo from "./holoGo";

// QUIC
const network = {
  network_type: NetworkType.QuicBootstrap,
  transport_pool: [{ type: TransportConfigType.Quic }],
  bootstrap_service: "https://bootstrap-staging.holo.host/",
};

const config = Config.gen({ network });

holoGo(config);
