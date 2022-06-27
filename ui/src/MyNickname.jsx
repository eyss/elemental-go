import { StoreSubscriber } from 'lit-svelte-stores';
import * as React from 'react';
import { useReadable } from 'react-use-svelte-store';

const useStore = (store) =>
  useController(React, (host) => new StoreSubscriber(host, () => store));

export default function ({ store }) {
  const myProfile = useReadable(store.myProfile);
  store.myProfile.subscribe(console.log);
  return <span>Hi {myProfile?.nickname}!</span>;
}
