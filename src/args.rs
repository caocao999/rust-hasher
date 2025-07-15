use clap::{Arg, Command};

pub struct CliArgs {
    pub file: String,
    pub hash: String,
    pub output: Option<String>,
}

pub fn parse_args() -> CliArgs {
    let matches = Command::new("hash")
        .version("0.1.0")
        .author("Your Name")
        .about("Calculate hash of a file")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .required(true)
                .value_name("FILE")
                .help("Path to the file"),
        )
        .arg(
            Arg::new("hash")
                .short('H')
                .long("hash")
                .default_value("sha256")
                .help("Hash algorithm (e.g., sha256, sha512, md5)")
                .value_name("HASH"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output file path (default: stdout)")
                .value_name("OUTPUT"),
        )
        .get_matches();

    CliArgs {
        file: matches.get_one::<String>("file").unwrap().to_string(),
        hash: matches.get_one::<String>("hash").unwrap().to_string(),
        output: matches.get_one::<String>("output").map(|s| s.to_string()),
    }
}
