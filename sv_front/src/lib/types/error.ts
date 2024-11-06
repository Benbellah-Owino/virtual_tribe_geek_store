export enum PageError{
    SeverError,
    ConnectionTimedOut,
    NotFoundError,
    Unauthorized
}

export enum FormError{
    PasswordsDontMatch,
    MissingField,
    SubmissionFailed,
    UpdateFailed
}

export type FieldError = {
    field: string,
    message: string
}