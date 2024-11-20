export type SurrealId = {
    [key: string] : string | Id | (()=>string);
    id: Id,
    tb: string
}

type Id = {
    [key: string] : string;
    String: string,
}

export function suridToString(surrealId: SurrealId):string{
    return `${surrealId.tb}:${surrealId.id.String}`
}