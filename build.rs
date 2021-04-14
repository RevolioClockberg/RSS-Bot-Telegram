use std::env;
use std::path::Path;

use ctl10n;

const LOCALES: &[&str] = &["en", "fr"];

// This file use the crate https://crates.io/crates/ctl10n
// to use embed messages which are declared in ./locales/*
// The language file is choose according to "LOCALE" environment variable.
// By default English is chosen. 

fn main() {
    for locale in LOCALES {
        println!("cargo:rerun-if-changed=locales/{}.toml", locale);
    }
    println!("cargo:rerun-if-env-changed=LOCALE");
    let locale_file = format!(
        "locales/{}.toml",
        &env::var("LOCALE").unwrap_or("en".to_string())
    );
    let out_file = Path::new(&env::var("OUT_DIR").unwrap()).join("ctl10n_macros.rs");
    let _ignore_error = std::fs::remove_file(&out_file);
    ctl10n::convert_strings_file(locale_file, out_file).expect("ctl10n failed");
}
