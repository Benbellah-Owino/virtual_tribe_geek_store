import type {  PageError } from "../error"
import { Result } from "../result"

export type PageState = {
    [key:string]: Result| string | PageError |null| boolean;
    inner_state: Result;
    error: PageError | null;
    loading: boolean,
    message: string
}


export function updatePageState(pageState: PageState,innerState: Result, error: PageError | null, loading: boolean,message: string){
    pageState.inner_state = innerState;
    pageState.error = error;
    pageState.loading = loading;
    pageState.message = message;
}