import { writable, type Writable } from "svelte/store";
const logStore :Writable<{
    logs:string[]
}> = writable({
    logs: ["Athena OS GUI Installer."]
  });
  
  export default logStore;
  