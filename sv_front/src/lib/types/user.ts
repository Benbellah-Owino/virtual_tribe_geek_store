import type { Socials } from "./creator";

export interface User{
    [key:string] : string | string[]|null| Socials | Date;
    username: string,
    email: string,
    description: string|null,
    avatar: string|null,
    joined_at: Date|null
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
