import { useState, useEffect } from 'react';
//import logo from './logo.svg';
import './App.css';
/* import {
  profilesStoreContext,
  ProfilesStore,
} from '@holochain-open-dev/profiles';
import {
  ContextProvider,
  ListProfiles,
  SearchAgent,
  CreateProfile,
  CreateInvitation,
} from './elements'; */
//import { createMockZome } from './mock-zome';
//import { HolochainClient } from "@holochain-open-dev/cell-client";
//import { InvitationsStore, invitationsStoreContext } from "@eyss/invitations";
import * as name from '@holochain-open-dev/profiles';
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
    });
  }
}

function App() {
  const [store, setStore] = useState(undefined);
  const [view, setView] = useState(`register`);
  
  console.log("name ", name)
  
  //console.log(context.nombre)

  async function connect() {
    const client = await HolochainClient.connect(
      `ws://localhost:${process.env.REACT_APP_HC_PORT}`,
      'elemental-go'
    );
    const cellClient = client.forCell(
      client.cellDataByRoleId('elemental-go')
    );
   /*  var profilesStores = new ProfilesStore(cellClient);
    var invitations = new InvitationsStore(cellClient, profilesStores);
    return {
      profilesStores,
      invitations
    } */
  }

 

  useEffect(() => {
    connect().then((store) => {
      setStore(store);
    });
  });
  console.log("Store", store);


  if (!store) {
    return <span>Loading...</span>;
  }

  console.log("Val Store ", store);
  store.knownProfiles.subscribe((allProfiles) => {
    //console.log(allProfiles)
    Object.keys(allProfiles).length > 0 ? setView('List-profile') : {};
  });

  WindowsViews()
/* 
  return (
    <div>
      <ContextProvider context={profilesStoreContext} value={store.profilesStores}>
        <ContextProvider context={invitationsStoreContext} value={store.invitations}>
          {
            view == 'register' ?
              <CreateProfile
              
              ></CreateProfile> :
              <>
                <ListProfiles
                  
                  onagentselected={(e) => alert(e.detail.agentPubKey)}
                ></ListProfiles>
                <SearchAgent
                  
                  includeMyself={true}
                ></SearchAgent>

                <CreateInvitation
                  

                  style={{ height: '600px' }}
                //onagentselected={(e) => alert(e.detail)}
                //
                ></CreateInvitation>
              </>

          }

        </ContextProvider>
      </ContextProvider>
    </div>
  ); */
}

export default App;

