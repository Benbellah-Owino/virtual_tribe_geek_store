import type { SurrealId } from "./server";

export interface ContentForCreate{
    [key:string]: string | string[]|  number | SurrealId |Genre[];
    title: string,
    studio: string,
    description: string,
    audiences: string,
    recom_price: number | string,
    genre: string[],
}


export interface Content extends Omit<ContentForCreate, "studio"|"genre">{
    [key:string]: string | string[]|  number | SurrealId | Genre[];
    id: SurrealId,
    studio: SurrealId,
    rating: number,
    genre: Genre[]
}

export interface Genre{
    [key:string]: string | SurrealId;
    id: SurrealId,
    name: string,
    description: string,
}