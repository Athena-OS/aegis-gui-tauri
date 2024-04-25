import { writable, type Writable } from "svelte/store";
const logStore :Writable<{
    logs:string[],
    progress: number,
    installFailed: boolean,
    installSuccess: boolean
}> = writable({
    logs: ["Athena OS GUI Installer.","Athena OS GUI Installer.","Athena OS GUI Installer.","Athena OS GUI Installer.","Athena OS GUI Installer.","Athena OS GUI Installer.","Athena OS GUI Installer.","Athena OS GUI Installer.","Athena OS GUI Installer."],
    progress: 0,
    installFailed:false,
    installSuccess:true
  });
  
  export default logStore;
  