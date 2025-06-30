export enum ComicType{
    ZIP,
    OCTET_STREAM,
    PDF,
    UNKNOWN
}

export function ret_comic_type(content_type: string) : ComicType{
    switch (content_type.toLowerCase()) {
        case "zip":
            return ComicType.ZIP
        case "octet-stream":
            return ComicType.OCTET_STREAM
        case "pdf":
            return ComicType.PDF
    
        default:
            return ComicType.UNKNOWN
    }
}