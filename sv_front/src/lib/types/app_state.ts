import type { FormError } from "./error"
import { Result } from "./result"

export type FormState = {
    [key:string]: Result| string | FormError |null;
    inner_state: Result;
    error: FormError | null;
    message: string;
    target: string;
}


export function updateFormState(formState: FormState, result: Result, error: FormError, message: string, target: string){
            formState.inner_state = result;
            formState.error= error;
            formState.message =  message;
            formState.target = target;
}

export function clearState(formState: FormState) {
            formState.inner_state = Result.Ok;
            formState.error =  null;
            formState.message =  '';
            formState.target = '';

}