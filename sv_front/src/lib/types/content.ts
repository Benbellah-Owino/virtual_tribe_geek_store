import type { SurrealId } from "./server";

export interface ContentForCreate{
    [key:string]: string | string[]|  number | SurrealId;
    title: string,
    studio: string,
    description: string,
    audiences: string,
    recom_price: number | string,
    genre: string[],
}


export interface Content extends Omit<ContentForCreate, "studio">{
    [key:string]: string | string[]|  number | SurrealId;
    id: SurrealId,
    studio: SurrealId,
    rating: number
}

export interface Genre{
    [key:string]: string | SurrealId;
    id: SurrealId,
    name: string,
    description: string,
}