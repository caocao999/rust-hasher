use sha2::{Sha256,Sha512,Digest};

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
