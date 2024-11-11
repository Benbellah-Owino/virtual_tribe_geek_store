export interface Studio{
    [key:string]: string;
    name:string,
    owner:string,
    email:string,
    description:string,
}

export interface StudioForCreate extends Studio{
    [key:string]: string;
}
export interface StudioCreator{
    [key:string]: string;
    studio:string,
    creator: string,
    role: string,
}