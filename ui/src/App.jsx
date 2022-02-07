import { useState, useEffect } from 'react';
import logo from './logo.svg';
import './App.css';
import {
  profilesStoreContext,
  ProfilesStore,
} from '@holochain-open-dev/profiles';
import {
  ContextProvider,
  ListProfiles,
  SearchAgent,
  CreateProfile,
} from './elements';
import { createMockZome } from './mock-zome';
import { HolochainClient } from "@holochain-open-dev/cell-client";


function App() {
  const [store, setStore] = useState(undefined);

  async function connect() {
    const client = await HolochainClient.connect(
      `ws://localhost:${process.env.VITE_HC_PORT}`,
      'elemental-go'
    );
    const cellClient = client.forCell(
      client.cellDataByRoleId('profiles')
    );
    return new ProfilesStore(cellClient);
  }
  useEffect(() => {
    connect().then((store) => {
      setStore(store);
    });
  });

  if (!store) {
    return <span>Loading...</span>;
  }

  return (
    <div>
      <ContextProvider context={profilesStoreContext} value={store}>
        <CreateProfile></CreateProfile>
        <ListProfiles
          onagentselected={(e) => alert(e.detail.agentPubKey)}
        ></ListProfiles>
        <SearchAgent includeMyself={true}></SearchAgent>
      </ContextProvider>ro
    </div>
  );
}

export default App;
