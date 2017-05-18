extern crate clap;
extern crate pandoc_ast;
extern crate tools;

use std::io::{self, Read, Write};
use pandoc_ast::{Block, Inline, MetaValue};

fn main() {
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .args_from_usage("--level=<level>")
        .get_matches();
    let level = matches.value_of("level").unwrap()
        .parse().unwrap();
    let mut json = String::default();
    io::stdin().read_to_string(&mut json).unwrap();
    io::stdout().write_all(&pandoc_ast::filter(json, |mut pandoc| {
        {
            let title = pandoc.meta.get("biblio-title");
            if let Some(&MetaValue::MetaString(ref title)) = title {
                pandoc.blocks.push(Block::Header(
                    level,
                    Default::default(),
                    vec![Inline::Str(title.clone())]));
            };
        }
        pandoc
    }).as_bytes()).unwrap();
}
