import type { User, UserForCreate } from "./user"

export interface Creator extends User{
    role: string,
    socials: Socials
}

export type Socials = {
    [key: string] : string| null,
    twitter_x: string|null,
    instagram: string|null,
    facebook: string|null
}


export interface CreatorForCreate extends UserForCreate{
    role: string[]| string,
}