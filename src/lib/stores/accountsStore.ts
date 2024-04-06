import { writable, type Writable } from "svelte/store";

const accountsStore: Writable<{
    users: {
        name: string;
        username: string;
        password: string;
        hasroot: boolean;
    }[],
    createNewUserTemp:{
        name:string;
        username: string;
        password: string;
        confirmPassword: string;
        passwordSameAsRoot: boolean;
        hasroot: boolean;
        isEditing: boolean;
    },
    rootpass: string
}> = writable({
    users: [],
    createNewUserTemp: {
        name:"",
        username:"",
        password:"",
        confirmPassword:"",
        passwordSameAsRoot:false,
        hasroot: false,
        isEditing: false
    },
    rootpass: ""
});

export default accountsStore;
