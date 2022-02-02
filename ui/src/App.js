import logo from './logo.svg';
import './App.css';


import React, { useState } from 'react';
import BrowserRouterApp from './Router/Router'
import Table from './Routes/table';

function App() {

  const [_activeGameHash, Set_activeGameHash] = useState(undefined);
  const [_gameEnded, Set_gameEnded] = useState(false);
  const [loading, Set_loading] = useState(false);
  const [_signedIn, Set_signedIn] = useState(false);
  

  /*  
    @state()
    _activeGameHash: string | undefined = undefined;
  
    @property()
    _gameEnded: boolean = false;
  
    @state()
    _loading = true;
  
    @state()
    _signedIn = false;
  */
  return (
      <BrowserRouterApp />

  );
}

export default App;
