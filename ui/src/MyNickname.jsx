import { StoreSubscriber } from "lit-svelte-stores";
import * as React from "react";
import { useController } from "@lit-labs/react/use-controller.js";

const useStore = (store) =>
  useController(React, (host) => new StoreSubscriber(host, () => store));

export default function ({ store }) {
  const myProfile = useStore(store.myProfile);

  return <span>Hi {myProfile.value?.nickname}!</span>;
}
