import { Routes, Route } from 'react-router-dom'
import { useState, useEffect } from 'react';

import { createMockZome } from './mock-zome';

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


const BrowserRouterApp = function () {
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

    return <>
        <ContextProvider context={profilesStoreContext} value={store}>
            <CreateProfile></CreateProfile>
            <ListProfiles
                onagentselected={(e) => alert(e.detail.agentPubKey)}
            ></ListProfiles>
            <SearchAgent includeMyself={true}></SearchAgent>
        </ContextProvider>

    </>
}

export default BrowserRouterApp;