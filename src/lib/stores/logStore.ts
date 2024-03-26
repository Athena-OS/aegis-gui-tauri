import { writable, type Writable } from "svelte/store";
const logStore :Writable<{
    logs:string[],
    progress: number,
    installFailed: boolean
}> = writable({
    logs: ["Athena OS GUI Installer."],
    progress: 0,
    installFailed:false
  });
  
  export default logStore;
  