import type { SurrealId } from "./server";

export interface StudioForCreate{
    [key:string]: string | SurrealId;
    name:string,
    owner:string,
    email:string,
    description:string,
}

export interface Studio extends Omit<StudioForCreate, "owner">{
    [key:string]: string | SurrealId;
    id: SurrealId,
    owner: SurrealId,
}
export interface StudioCreator{
    [key:string]: string;
    studio:string,
    creator: string,
    role: string,
}