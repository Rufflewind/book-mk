extern crate serde_json;
extern crate tempdir;
extern crate toml;

use std::ffi::{OsStr, OsString};
use std::io::{Read, Write};
use std::path::Path;

pub mod mdbook {
    extern crate mdbook;

    use std::error::Error;
    use std::path::{Component, Path};
    use tempdir;

    /// Fetch the item tree of a book.
    ///
    /// Don’t forget to initialize `env_logger`.
    pub fn get_items(src_dir: &Path) -> Result<Vec<mdbook::BookItem>,
                                               Box<Error>> {
        // use a temp dir for dest dir in case mdbook decides to purge the
        // directory in the future for whatever reason
        let tmp_dir = tempdir::TempDir::new("")?;

        let mut book = mdbook::MDBook::new(Path::new(&Component::CurDir))
            .set_src(src_dir)
            .set_dest(tmp_dir.path());

        // parse and populate the book items; unfortunately this also has the
        // side-effect of creating root, dest, and src (despite create_missing
        // == false), so that's why we set the root to "." and dest to "target"
        //
        // if only parse_summary was a public method!
        //
        // note: this will create missing .md files since we
        //       did not set book.create_missing to false
        book.init()?;

        Ok(book.content)
    }

    /// Iterate over each item of the book, yielding `mdbook::BookItem` at
    /// every iteration.
    pub fn iter_items(items: &[mdbook::BookItem])
                      -> mdbook::book::BookItems {
        mdbook::book::BookItems {
            items: &items,
            current_index: 0,
            stack: Default::default(),
        }
    }

    pub fn find_item_by_path<'a>(book_items: &'a [mdbook::BookItem],
                                 path: &Path)
                                 -> Option<&'a mdbook::BookItem> {
        for item in iter_items(&book_items) {
            match item {
                &mdbook::BookItem::Chapter(_, ref chapter) => {
                    if chapter.path == path {
                        return Some(item);
                    }
                }
                &mdbook::BookItem::Affix(ref chapter) => {
                    if chapter.path == path {
                        return Some(item);
                    }
                }
                &mdbook::BookItem::Spacer => {}
            }
        }
        None
    }

    pub fn chapter_depth(number: &str) -> i64 {
        number.chars().filter(|&c| c == '.').count()
            .checked_sub(1).unwrap() as _
    }
}

pub mod pandoc {
    extern crate pandoc_ast;

    use std;
    use serde_json;

    pub fn empty() -> pandoc_ast::Pandoc {
        pandoc_ast::Pandoc {
            meta: Default::default(),
            blocks: Default::default(),
            pandoc_api_version: vec![1, 17, 0, 5],
        }
    }

    pub fn from_json(json: String) -> pandoc_ast::Pandoc {
        // 'filter' has some extra error-checking beyond the Deserialize impl
        let mut pandoc = empty();
        pandoc_ast::filter(json, |p| std::mem::replace(&mut pandoc, p));
        pandoc
    }

    pub fn to_json(pandoc: &pandoc_ast::Pandoc) -> String {
        serde_json::to_string(pandoc).expect("unreachable")
    }
}

pub mod quote {
    pub fn yaml(s: &str) -> String {
        "'".to_owned() + &s.replace("'", "'''") + "'"
    }
}

pub fn add_os_str<S1, S2>(s1: S1, s2: S2) -> OsString
    where S1: Into<OsString>,
          S2: AsRef<OsStr>,
{
    let mut s1 = s1.into();
    s1.push(s2);
    s1
}

pub fn load_file(path: &Path) -> std::io::Result<String> {
    let mut s = String::new();
    std::fs::File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

pub fn save_file(path: &Path, contents: &str) -> std::io::Result<()> {
    let mut f = std::fs::File::create(path)?;
    f.write_all(contents.as_bytes())?;
    Ok(())
}
