use sha2::{Sha256,Sha512,Digest};
use std::fs::File;
use std::io::Write;



pub fn hash256(data:&Vec<u8>)->String{
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let result = hasher.finalize();
    format!("{:x}",result)
}

pub fn hash512(data:&Vec<u8>)->String{
    let mut hasher = Sha512::new();
    hasher.update(&data);
    let result = hasher.finalize();
    format!("{:x}",result)
}

pub fn output_file(file_path:&str,data:&str) -> Result<(),String>{
    let mut file = File::create(file_path).map_err(|_e_| "ハッシュ値書き込みようファイルが開けません".to_string())?;
    file.write(data.as_bytes()).map_err(|_e_| "ハッシュ値が書き込みめません".to_string())?;
    Ok(())
}