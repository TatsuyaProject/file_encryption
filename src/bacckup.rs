use std::{fs::File, path::PathBuf};

use thiserror::Error;

// @TODO Add) Offset

#[derive(Debug,Error)]
pub enum FileError {
    #[error("[failed] setting paths")]
    NullOfPath,

}

struct Folder{
    paths: Vec<Box<Path>>,
    names: Vec<File>,
}
impl Folder {
    pub fn new(paths: Vec<Box<Path>>) -> Self{
        let names: Vec<File>= Vec::new();
        Self { paths, names }
    }
}

fn searching_folders() -> Result<Vec<Box<Path>>, FileError>{
    
}

fn get_windows_path() -> Result<String, FileError>{

}

fn main() {
    let mut who_am_i: String;
    let mut path: Result<String, FileError>;
    let mut path;

    if cfg!(windows) {
        println!("Running on Windows");
        who_am_i= match whoami::realname(){
            Ok(string) => string,
            Err(err) => err.to_string()
        };

        path= match get_windows_path() {
            Ok(string) => {
                path = PathBuf::from("C:/");
                path.push("/Users");
                path.push("/{}", who_am_i);
                path
            },
            Err(FileError::NullOfPath) => {
                panic!("files are null");
            },
        };



    }
    
    if cfg!(Linux){
        println!("Running on Linux");
        // let path= Path::new("~/");
    }
    
    // let box_path: Box<Path>= Box::from(path);
    // let mut v_paths: Vec<Box<PathBuf>>= Vec::new();
    
    // let v_paths= searching_folders();
    
    // for _path in v_paths{
    //     let folder= Folder::new(v_paths);
    // }
}