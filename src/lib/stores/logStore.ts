import { writable, type Writable } from "svelte/store";
const logStore :Writable<{
    logs:string[],
    progress: number,
    installFailed: boolean,
    installSuccess: boolean
}> = writable({
    logs: [],
    progress: 0,
    installFailed:false,
    installSuccess:false
  });
  
  export default logStore;
  