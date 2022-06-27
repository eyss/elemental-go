import { useState, useEffect } from 'react';
import logo from './logo.svg';
import './App.css';
import {
  profilesStoreContext,
  ProfilesStore,
} from '@holochain-open-dev/profiles';

import{
  AppWebsocket,
  CallZomeRequest
} from '@holochain/client';

import {
  ContextProvider,
  ListProfiles,
  SearchAgent,
  CreateProfile,
} from './elements';
import { createMockZome } from './mock-zome';
import MyNickname from './MyNickname';

function App() {
  const [store, setStore] = useState(undefined);

  async function connect() {
    const cellClient = await createMockZome();
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

  //console.log("profilesStoreContext", profilesStoreContext)
  console.log("store ", store)

  return (
    <div>
      <ContextProvider context={profilesStoreContext} value={store}>
        <CreateProfile></CreateProfile>
        <ListProfiles
          onagentselected={(e) => alert(e.detail.agentPubKey)}
        ></ListProfiles>
        <SearchAgent includeMyself={true}></SearchAgent>
      </ContextProvider>
      <MyNickname store={store}></MyNickname>
    </div>
  );
}

export default App;
