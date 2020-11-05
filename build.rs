use std::{env, fs::copy, io::Error, path::Path, process::exit};

const ENGLISH_WORDS_DIR: &str = "english_words";
const WORDS_ALPHA_FILENAME: &str = "words_alpha.txt";

fn main() -> Result<(), Error> {
    let words_dir = Path::new(ENGLISH_WORDS_DIR);
    if !words_dir.exists() {
        eprintln!(
            "'{}' directory not found, init git submodules",
            ENGLISH_WORDS_DIR
        );
        exit(1);
    }
    let words_dir_file_name = format!("{}/{}", ENGLISH_WORDS_DIR, WORDS_ALPHA_FILENAME);
    let words_file = Path::new(&words_dir_file_name);
    if !words_file.exists() {
        eprintln!(
            "'{}' file not found, init git submodules",
            words_dir_file_name
        );
        exit(1);
    }
    let out_dir = env::var("OUT_DIR").expect("Could not get 'OUT_DIR' env var");
    copy(words_file, Path::new(&out_dir).join(WORDS_ALPHA_FILENAME)).expect("Could not copy file");

    println!("cargo:rerun-if-changed=english_words");
    Ok(())
}
