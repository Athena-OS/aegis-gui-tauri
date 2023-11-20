import { writable } from "svelte/store";

const currentUserStore = writable({
  fullname: "",
  username: "",
  password: "",
  permissions: {
    allowRootAccess: false,
    permission2: false,
    permission3: false,
    permission4: false,
    permission5: false,
  },
});

export default currentUserStore;
