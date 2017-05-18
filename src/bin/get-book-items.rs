extern crate clap;
extern crate env_logger;
extern crate mdbook;
extern crate tools;

use std::path::Path;

fn main() {
    env_logger::init().unwrap();
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .args_from_usage("<src-dir>")
        .get_matches();
    let src_dir = matches.value_of("src-dir").unwrap();
    let book_items = tools::mdbook::get_items(Path::new(&src_dir)).unwrap();
    for item in tools::mdbook::iter_items(&book_items) {
        match item {
            &mdbook::BookItem::Chapter(_, ref chapter) => {
                println!("{}\n", chapter.path.display());
            }
            &mdbook::BookItem::Affix(ref chapter) => {
                println!("{}\n", chapter.path.display());
            }
            &mdbook::BookItem::Spacer => {}
        }
    }
}
