import type { Socials } from "./creator";

export interface User{
    [key:string] : string | string[]|null| Socials;
    username: string,
    email: string,
    description: string|null,
    password: string,
}

export interface UserForCreate{
    [key:string] : string |string[]| null| Socials;
    username: string,
    email: string,
    password: string,
    confirm_password: string,
}

export interface UserForLogin{
    [key:string] : string ;
    email: string,
    password: string,
}
