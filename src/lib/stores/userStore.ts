import { writable } from "svelte/store";

function createUserStore() {
  let users: any = [];
  const { subscribe, update } = writable(users);

  return {
    subscribe,
    addUser: () => update(() => (users = [...users])),
    removeUser: (index: number) =>
      update(
        () => (users = [...users.slice(0, index), ...users.slice(index + 1)])
      ),
  };
}
export const userStore = createUserStore();
