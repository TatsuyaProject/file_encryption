use std::path::PathBuf;
use thiserror::Error;
use seekable_stream_cipher::keccak::StreamCipher;
use std::fs::File;
use std::io::Read;

#[derive(Debug,Error)]
pub enum FileError {
    #[error("[failed] setting paths")]
    NullOfPath,
    #[error("[failed] generating key")]
    NotVaildKey
}

struct InFile{
    title: String,
    data_buff: Vec<u8>,
    encrypted: [u8; usize::MAX],
}
impl InFile{
    fn new(title: String, data_buff: Vec<u8>) -> Self{
        let encrypted= [0_u8; usize::MAX]; 
        Self { title, data_buff, encrypted }
    }
}

fn encrypting(array: [u8; 32], file_name: &str) -> Result<(), std::io::Error>{    
    let mut key = [0u8; StreamCipher::KEY_LENGTH];

    let file_data= File::open(file_name)?;

    getrandom::fill(&mut key).unwrap();
    let st = StreamCipher::new(&key, b"fill test");

    let mut msg = [0u8; 10000];
    getrandom::fill(&mut msg).unwrap();
    
    let mut msg2 = msg.clone();
    
    st.apply_keystream(&mut msg2[5..500], 5);

    Ok(())
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

        // keep encrypted string buff -> buff free;
    // }
    Ok(())
}
