use std::fs;
use std::process;
mod args;
mod my_lib;

fn main() {
    let args:args::CliArgs = args::parse_args();    

    let data = match fs::read(&args.file){
        Ok(data) => data,
        Err(err) => {
            eprintln!("{} {}",err,&args.file);
            process::exit(1);
        }
    };

    let hash = &args.hash[..];
    let result = match hash {
        "sha512" => my_lib::hash512(&data),
        _ => my_lib::hash256(&data),

    };


    //----------test---------------
    println!("{:?}",data);
    println!("{}",result);

    

   
}