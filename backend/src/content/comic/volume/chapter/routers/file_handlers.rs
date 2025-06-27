use std::{path};
use chrono::OutOfRangeError;
use http::StatusCode;
use mime_guess::Mime;
use tokio::{fs, io};
use std::fs::File;
use zip::{result::ZipError, ZipArchive};

#[derive(Debug)]
pub enum ComicFileError{
    FileOpenError(io::Error),
    ArchiveExtractionError(ZipError),
    OutOfRangeError
}


impl From<io::Error> for ComicFileError {
    fn from(value: io::Error) -> Self {
        ComicFileError::FileOpenError(value)
    }
}

impl From<ZipError> for ComicFileError {
    fn from(value: ZipError) -> Self {
        ComicFileError::ArchiveExtractionError(value)
    }
}

pub fn open_octet_stream(archive_path: String, index: usize) -> Result<(Vec<u8>, Mime), ComicFileError>{
    let file = File::open(&archive_path)?; 

    let mut archive = ZipArchive::new(file) ?;

    if index >= archive.len() {
        return Err(ComicFileError::OutOfRangeError)
    }

    let mut file = archive.by_index(index).unwrap();
    let mut buf = vec![];
    use std::io::Read;
    file.read_to_end(&mut buf).unwrap();

    let mime = mime_guess::from_path(file.name()).first_or_octet_stream();

    Ok((buf, mime))
}

pub async fn open_pdf(pdf_path: String) -> Result<Vec<u8>, ComicFileError>{
    let file = fs::read(&pdf_path).await?;

    return Ok(file)
}