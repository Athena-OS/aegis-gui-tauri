import { writable, type Writable } from "svelte/store";

const accountsStore: Writable<{
    users: {
        name: string;
        userName: string;
        password: string;
        hasRoot: boolean;
    }[],
    createNewUserTemp:{
        name:string;
        userName: string;
        password: string;
        confirmPassword: string;
        passwordSameAsRoot: boolean;
        hasRoot: boolean;
        isEditing: boolean;
    },
    rootpass: string
}> = writable({
    users: [],
    createNewUserTemp: {
        name:"",
        userName:"",
        password:"",
        confirmPassword:"",
        passwordSameAsRoot:false,
        hasRoot: false,
        isEditing: false
    },
    rootpass: ""
});

export default accountsStore;
