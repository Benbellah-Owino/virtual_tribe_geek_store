export type SurrealId = {
    [key: string] : string | Id;
    id: Id,
    table: string
}

type Id = {
    [key: string] : string;
    String: string,
}