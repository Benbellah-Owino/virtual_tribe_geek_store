import type { SurrealId } from "./server";
import type { Studio } from "./studio";

export interface ContentForCreate{
    [key:string]: string | string[]|  number | SurrealId |Genre[] |Studio| null;
    title: string,
    studio: string,
    description: string,
    audiences: string,
    recom_price: number | string,
    genre: string[],
}


export interface Content extends Omit<ContentForCreate, "studio"|"genre">{
    [key:string]: string | string[]|  number | SurrealId | Genre[]| Studio |null;
    id: SurrealId,
    studio: Studio|null,
    rating: number,
    genre: Genre[]
}

export interface Genre{
    [key:string]: string | SurrealId;
    id: SurrealId,
    name: string,
    description: string,
}


export interface ComicForCreate{
    [key:string]: string | string[]|  number |SurrealId|  SurrealId[] |null;
    content: string| SurrealId,
    writer: string[],    
    artist: string[],
    creator: SurrealId[] | string[],
}
export interface VolumeForCreate{
    [key:string]: string | SurrealId| null | number;
    synopsis: string,
    comic: SurrealId,
    cover: null|string,
    no_of_chapters: number
}