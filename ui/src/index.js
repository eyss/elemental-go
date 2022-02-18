/* // Adding the profile elements we need
import '@holochain-open-dev/profiles/create-profile';
import '@holochain-open-dev/profiles/list-profiles';
import '@holochain-open-dev/profiles/search-agent';

// Add the context-provider element
import '@holochain-open-dev/context/context-provider'; */

import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './App';

//customElements.define('create-invitation', CreateInvitation)

ReactDOM.render(
  //<React.StrictMode>
    <App />,
  //</React.StrictMode>,
  document.getElementById('root')
);
