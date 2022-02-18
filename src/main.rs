#![feature(iter_advance_by)]
mod argopt;
use crate::argopt::Encoding;
use argopt::Opt;
use sha2::Digest;
use std::{fs::File, io::BufRead, io::BufReader, thread};
use structopt::StructOpt;

macro_rules! thread_ripper {
    ($thread:expr, $seg:expr, $hash:expr, $word:expr) => {
        for i in 0..$thread {
            let seg = $seg.clone();
            let hash = $hash.clone();
            let word = $word.clone();
            let handle = thread::spawn(move || {
                let init = i * seg;
                let end = seg - 1;
                //get the file
                let wordlist_file = File::open(word).unwrap();
                let reader = BufReader::new(wordlist_file);
                let mut reader_lines = reader.lines();
                let _ = reader_lines.advance_by(init);
                let mut count = 0;
                for line in reader_lines {
                    if count <= end {
                        // dbg!(count);
                        let line = line.unwrap();
                        let word_pass = line.trim();
                        let h = &hex::encode(sha2::Sha256::digest(&word_pass.as_bytes()));
                        if hash == String::from(h) {
                            println!("Success!!!",);
                            println!("Password found by thread {i}: {}", word_pass);
                            break;
                        }
                        count += 1;
                    }
                }
            });
            handle.join().unwrap();
        }
    };
}
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
        Encoding::SHA1 => thread_ripper!(thnum, segment, hash, wordlist),
        Encoding::SHA256 => thread_ripper!(thnum, segment, hash, wordlist),
        Encoding::SHA512 => thread_ripper!(thnum, segment, hash, wordlist),
    }
}
