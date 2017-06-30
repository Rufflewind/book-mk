extern crate clap;
extern crate tools;

use std::io::{Read, Write};
use std::path::Path;

fn main() {
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .args_from_usage("--output-ext=[extension]")
        .args_from_usage("--output-dir=[dir]")
        .get_matches();
    let output_ext = matches.value_of("output-ext").unwrap();
    let output_dir = Path::new(matches.value_of_os("output-dir")
                               .unwrap_or(Default::default()));
    let mut amalg = String::default();
    std::io::stdin().read_to_string(&mut amalg).unwrap();
    let mut current = None;
    for line in amalg.lines() {
        match tools::match_amalg_prefix(line) {
            Some(n) => {
                let path = output_dir.join(n).with_extension(output_ext);
                current = Some(std::fs::File::create(path).unwrap());
            }
            None => {
                let mut f = current.as_mut()
                    .expect("invalid amalgamation file");
                f.write_all(line.as_bytes()).unwrap();
                f.write_all(b"\n").unwrap();
            }
        }
    }
}
