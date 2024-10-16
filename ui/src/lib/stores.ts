import { writable } from "svelte/store";

function authInit() {
  const { subscribe, set, update } = writable(false);

  return {
    subscribe,
    login: () => update((n) => (n = true)),
    logout: () => set(false),
  };
}

export const auth = authInit();
