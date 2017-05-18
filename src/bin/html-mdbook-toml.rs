extern crate clap;
extern crate toml;
extern crate tools;

use std::collections::BTreeMap;

fn oxford_series<I>(items: I) -> String
    where I: IntoIterator,
          I::IntoIter: ExactSizeIterator,
          I::Item: AsRef<str>,
{
    let items = items.into_iter();
    let len = items.len();
    let mut s = String::new();
    for (i, x) in items.enumerate() {
        if i == len - 1 {
            if len == 2 {
                s.push_str(" and ");
            } else if len != 1 {
                s.push_str(", and ");
            }
        } else {
            s.push_str(", ");
        }
        s.push_str(x.as_ref());
    }
    s
}

fn main() {
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .args_from_usage("<dest-dir>")
        .args_from_usage("-M, --metadata=<metadata> ...")
        .get_matches();
    let dest_dir = matches.value_of("dest-dir").unwrap();
    let metadata = matches.values_of("metadata").unwrap();
    let mut config = BTreeMap::default();
    let mut authors = Vec::default();
    for entry in metadata {
        let entry: Vec<_> = entry.split("=").collect();
        if entry.len() != 2 {
            panic!("metadata must be in the form -M key=value");
        }
        if entry[0] == "author" {
            authors.push(entry[1]);
        } else {
            config.insert(entry[0].to_owned(),
                          toml::Value::from(entry[1].to_owned()));
        }
    }
    if !authors.is_empty() {
        config.insert("author".to_owned(),
                      toml::Value::from(oxford_series(authors)));
    }
    config.insert("dest".to_owned(), toml::Value::from(dest_dir.to_owned()));
    print!("{}", toml::to_string(&toml::Value::from(config)).unwrap());
}
