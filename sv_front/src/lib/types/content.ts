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

export interface Volume{
    [key:string]: string | SurrealId| null | number;
    id: SurrealId
    synopsis: string,
    comic: SurrealId,
    cover: null|string,
    no_of_chapters: number
    //runlengtj: RunLength
    vol_no:number,
}

export interface ChapterForCreate{
    [key:string]: string | SurrealId| null | number;
    pages: number,
    title: string,
    synopsis: string,
    volume: SurrealId
}

export interface Chapter{
    [key:string]: string | SurrealId| null | number;
    pages: number,
    title: string,
    synopsis: string,
    volume: SurrealId,
    relative_chapter: number,
    absolute_chapter: number,
    cover: string|number
}

export interface ComicFileDetails{
    [key:string] : string | number;
    count: number,
    content_type: string
}


// Video
export interface VideoForCreate{
    [key:string]: string | string[]|  number |SurrealId|  SurrealId[] | RunLength | null;
    content: string| SurrealId,
    writer: string[],    
    creator: SurrealId[] | string[],
    video_type: string,
    average_run_length: RunLength
}

export interface RunLength{
    [key: string] : number,
        seconds: number,
        minutes: number,
        hours : number
}