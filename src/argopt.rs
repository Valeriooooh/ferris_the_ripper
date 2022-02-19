use std::{str::FromStr, string::ParseError};
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(
    name = "ferris_the_ripper",
    about = "A sha password cracking utility made in rust."
)]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    pub wordlist: std::path::PathBuf,

    #[structopt(short = "e", long = "encoding")]
    pub encoding: Encoding,

    #[structopt()]
    pub hash: String,

    #[structopt(short = "n")]
    pub thread_number: Option<usize>,
}

#[derive(Debug)]
pub enum Encoding {
    SHA1,
    SHA224,
    SHA256,
    SHA384,
    SHA512,
}

impl FromStr for Encoding {
    type Err = ParseError;
    fn from_str(day: &str) -> Result<Self, Self::Err> {
        let _ = day.to_lowercase();
        match day {
            "sha1" => Ok(Encoding::SHA1),
            "1" => Ok(Encoding::SHA1),
            "sha224" => Ok(Encoding::SHA224),
            "224" => Ok(Encoding::SHA224),
            "sha256" => Ok(Encoding::SHA256),
            "256" => Ok(Encoding::SHA256),
            "sha384" => Ok(Encoding::SHA384),
            "384" => Ok(Encoding::SHA384),
            "sha512" => Ok(Encoding::SHA512),
            "512" => Ok(Encoding::SHA512),
            _ => Ok(Encoding::SHA256),
        }
    }
}
