import type { SurrealId } from "$lib/types/server";

export function surrealIdToString(id: SurrealId): string{
    return `${id.tb}:${id.id.String}`
}