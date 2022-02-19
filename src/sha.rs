#[macro_export]
macro_rules! thread_ripper {
    ($thread:expr, $seg:expr, $hash:expr, $word:expr, Sha1) => {
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
                        let h = &hex::encode(sha1::Sha1::digest(&word_pass.as_bytes()));
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
    ($thread:expr, $seg:expr, $hash:expr, $word:expr, $sha:tt) => {
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
                        let h = &hex::encode(sha2::$sha::digest(&word_pass.as_bytes()));
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
