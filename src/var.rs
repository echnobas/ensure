use std::path::PathBuf;

use once_cell::sync::Lazy;
use regex::Regex;

pub static SANITIZER: Lazy<Regex> = Lazy::new(|| Regex::new("<.*?>").unwrap());

const DAT: &str = ".ensure";

pub static DAT_PATH: Lazy<PathBuf> = Lazy::new(|| {
    std::env::var("HOME")
        .map(PathBuf::from)
        .map(|mut p| {
            p.push(DAT);
            p
        })
        .unwrap()
});
