use axum::{
    body::Bytes,
    extract::{multipart::{Field, MultipartError}, Multipart},
};


use crate::file_upload::storage::{gen_file_name, append_to_disk};



// region:      --- Error type
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Error {
    MEDIARETRIEVALERROR,
    FileNameGenError,
    TooBig
}

impl From<MultipartError> for Error{
    fn from(error: MultipartError) -> Self{
        Error::TooBig
    }
}
// endregion:   --- Error type

// region:      --- Types
#[derive(Debug, PartialEq, Clone)]
pub struct MultField {
    pub name: String,
    pub content_type: String,
    pub data: Bytes,
}

impl MultField {
    pub fn new() {}

    ///
    /// TIP: TO be able to upload files bigger that 2mb make sure to disable the default file limit in your router
    /// 
    pub async fn from_field(mut field: Field<'_>) -> Result<MultField, MultipartError> {
        eprintln!("{:#?}", &field);
        let content_type = field.content_type().unwrap().to_string();
        let name = field.file_name().unwrap().to_string();
        let mut data = Vec::new();
        let mut count = 0 as f32;
        let pow = usize::pow(2, 20) as f32;
        println!("{pow}");
        while let Some(chunk) = field.chunk().await.map_err(|e| e)? {
            let len =  chunk.len() as f32;
            let mbs = len / pow;
            count = count + mbs;

            println!(
                "received {}b ({}mb) , total is {}mb", 
                chunk.len(),
                mbs,
                count
            );
            
            data.extend_from_slice(&chunk);
        }

        let data = Bytes::from(data);
        
        println!(
            "Length of `{}.{}` is {} bytes",
            name,
            content_type,
            data.len()
        );

        Ok(MultField {
            name,
            content_type,
            data, //  TODO: Handle video case
        })
    }

pub async fn from_field_big_file(mut field: Field<'_>, destination: String) {
        eprintln!("{:#?}", &field);
        let content_type = field.content_type().unwrap().to_string();
        let name = field.file_name().unwrap().to_string();
        let mut count = 0 as f32;
        let pow = usize::pow(2, 20) as f32;
        println!("{pow}");
        let dest = gen_file_name(destination, &name, &content_type).unwrap();
        while let Some(chunk) = field.chunk().await.unwrap() {
            let len =  chunk.len() as f32;
            let mbs = len / pow;
            count = count + mbs;

            println!(
                "received {}b ({}mb) , total is {}mb", 
                chunk.len(),
                mbs,
                count
            );
            
            let _ = append_to_disk((&dest, chunk));
        }
        println!("File \"{}.{}\" sized {}mbs is saved to disk", name, content_type,count);
    }
}
// endregion:   --- Types

pub async fn extract_image(mut multipart: Multipart) -> Result<MultField, Error> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let field = MultField::from_field(field).await?;
        return Ok(field);
    }
    Err(Error::MEDIARETRIEVALERROR)
}
pub async fn extract_big_image(mut multipart: Multipart) -> Result<(), Error> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let _field = MultField::from_field_big_file(field, "media".to_string()).await;
        return Ok(());
    }
    Err(Error::MEDIARETRIEVALERROR)
}

