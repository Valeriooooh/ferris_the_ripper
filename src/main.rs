#![feature(iter_advance_by)]
mod argopt;
mod sha;
use crate::argopt::Encoding;
use argopt::Opt;
use sha2::Digest;
use std::{fs::File, io::BufRead, io::BufReader, thread};
use structopt::StructOpt;

fn main() {
    let args = Opt::from_args();
    let thnum = {
        match args.thread_number {
            Some(a) => a,
            None => 1,
        }
    };
    let hash = args.hash;
    let wordlist = args.wordlist;
    let enc = args.encoding;

    let wordlist_file = File::open(&wordlist).unwrap();
    let reader = BufReader::new(&wordlist_file);
    let c = reader.lines().count();
    let segment = c / thnum;
    match enc {
        Encoding::SHA1 => thread_ripper!(thnum, segment, hash, wordlist, Sha1),
        Encoding::SHA224 => thread_ripper!(thnum, segment, hash, wordlist, Sha224),
        Encoding::SHA256 => thread_ripper!(thnum, segment, hash, wordlist, Sha256),
        Encoding::SHA384 => thread_ripper!(thnum, segment, hash, wordlist, Sha384),
        Encoding::SHA512 => thread_ripper!(thnum, segment, hash, wordlist, Sha512),
    }
}
