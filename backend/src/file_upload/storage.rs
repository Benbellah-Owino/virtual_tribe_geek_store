use std::{
    fs::{self, OpenOptions},
    io:: Write,
    path::{Path, PathBuf},
};

use axum::body::Bytes;
use super::small_file::MultField;


#[derive(Debug)]
pub enum Error {
    FileNameGenError,
    PathNotFound,
    FileCreationError,
    PathCreationError
}

pub async fn store(destination: Option<String>,name:String, file: MultField) -> Option<(PathBuf,String, Bytes)> {
    if let Some(p) = destination {
        let dest = p.clone();
        
        let file_path = gen_file_name(p, &name[..], &file.content_type).unwrap();
        return Some((file_path,dest, file.data))
    }
    return None;
}


pub fn gen_file_name(
    mut destination: String,
    file_name: &str,
    content_type: &String
) -> Result<PathBuf, Error> {
    destination.push('\\');
    destination.push_str(file_name);
    destination.push('.');
    let content_type: Vec<&str> = content_type.split('/').collect();

    destination.push_str(&content_type[1]);
    let path = Path::new(&destination).to_path_buf();
    println!("{:#?}", path);

    println!("Stored...");
    return Ok(path);
}


//TODO: Implement a way to save to disk irregadles of disk content
pub async fn save_to_disk(to_write: (&PathBuf, String, Bytes)) -> Result<(), Error> {
    println!("saving to disk...");
    if !Path::new(&to_write.1).exists(){
        match fs::create_dir_all(&to_write.1){
            Ok(_) => println!("path created"),
            Err(_) => return Err(Error::FileCreationError)
        }
    }else{}


    match OpenOptions::new().create(true).write(true).open(to_write.0) {
        Ok(mut f) => {
            f.write_all(&to_write.2).unwrap();
            println!("saved!");
            Ok(())
        }
        Err(e) => {
            eprintln!("FAILED TO SAVE TO DISK.\n{:#?}", e);
            Err(Error::FileCreationError)
        }
    }
}
pub fn append_to_disk(to_write: (&PathBuf, Bytes)) {
    println!("saving to disk...");

    match OpenOptions::new().append(true).create(true).open(to_write.0) {
        Ok(mut f) => {
            f.write_all(&to_write.1).unwrap();
            println!("saved!")
        }
        Err(e) => { 

            eprintln!("FAILED TO SAVE TO DISK.\n{:#?}", e)
        }
    }
}