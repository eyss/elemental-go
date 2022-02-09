import { useState, useEffect } from 'react';
//import logo from './logo.svg';
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
//import { createMockZome } from './mock-zome';
import { HolochainClient } from "@holochain-open-dev/cell-client";



function WindowsViews() {
  if (window.navigator && navigator.serviceWorker) {
    navigator.serviceWorker.getRegistrations()
      .then(function (registrations) {
        console.log("Registration ", registrations)
        for (let registration of registrations) {
          registration.unregister();
        }
      }).catch(err => {
        console.log("Error ", err)
      })
      ;
  }







}




function App() {
  const [store, setStore] = useState(undefined);
  const [view, setView] = useState(`register`);


  async function connect() {
    const client = await HolochainClient.connect(
      `ws://localhost:${import.meta.env.VITE_HC_PORT}`,
      'elemental-go'
    );
    const cellClient = client.forCell(
      client.cellDataByRoleId('elemental-go')
    );
    return new ProfilesStore(cellClient);
  }

  useEffect(() => {
    connect().then((store) => {
      console.log(store);
      setStore(store);
    });
  });

  if (!store) {
    return <span>Loading...</span>;
  }

  console.log("Val Store ", store);
  store.knownProfiles.subscribe((allProfiles) => {
    console.log(allProfiles)
    Object.keys(allProfiles).length > 0 ? setView('List-profile') : {};
  });

  //WindowsViews()

  return (
    <div>
      <ContextProvider context={profilesStoreContext} value={store}>
        {
          view == 'register' ?
            <CreateProfile ></CreateProfile> :
            <>
              <ListProfiles
                onagentselected={(e) => alert(e.detail.agentPubKey)}
              ></ListProfiles>
              <SearchAgent includeMyself={true}></SearchAgent>
            </>
        }




      </ContextProvider>
    </div>
  );
}

export default App;
