import type { SurrealId } from "$lib/types/server";

export function surrealIdToString(id: SurrealId): string{
    return `${id.tb}:${id.id.String}`
}

export function stringToSurrealId(id: string): SurrealId{
    const idArr = id.split(':');

    return {
        id: {String: idArr[1]},
        tb: idArr[0]
    }
}