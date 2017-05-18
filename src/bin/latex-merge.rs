extern crate clap;
extern crate env_logger;
extern crate mdbook;
extern crate pandoc_ast;
extern crate tools;

use std::path::Path;
use pandoc_ast::{Block, MutVisitor};

struct Filter {
    shift: i64,
}

impl MutVisitor for Filter {
    fn visit_block(&mut self, block: &mut Block) {
        match *block {
            Block::Header(ref mut level, _, _) => {
                *level += self.shift
            }
            _ => {}
        }
    }
}

enum Part {
    FrontMatter,
    MainMatter,
    Appendix,
    BackMatter,
}

/// Read the JSON file for each book item and piece them together.
fn merge_book_items(src_dir: &Path)
                    -> (pandoc_ast::Pandoc,
                        pandoc_ast::Pandoc,
                        pandoc_ast::Pandoc)
{
    let book_items = tools::mdbook::get_items(&src_dir).unwrap();
    let mut main_doc = tools::pandoc::empty();
    let mut before_doc = tools::pandoc::empty();
    let mut after_doc = tools::pandoc::empty();
    let mut part = Part::FrontMatter;
    for item in tools::mdbook::iter_items(&book_items) {
        let (doc, path, shift) = match *item {
            mdbook::BookItem::Chapter(ref number, ref chapter) => {
                match part {
                    Part::FrontMatter => {
                        main_doc.blocks.push(Block::RawBlock(
                            pandoc_ast::Format("tex".into()),
                            "\\mainmatter".into()));
                        part = Part::MainMatter;
                    }
                    Part::MainMatter
                        if chapter.name.starts_with("Appendix:") =>
                    {
                        main_doc.blocks.push(Block::RawBlock(
                            pandoc_ast::Format("tex".into()),
                            "\\appendix".into()));
                        part = Part::Appendix;
                    }
                    _ => {}
                }
                let shift = tools::mdbook::chapter_depth(number);
                (&mut main_doc, &chapter.path, shift)
            }
            mdbook::BookItem::Affix(ref chapter) => {
                let doc = match part {
                    Part::FrontMatter => &mut before_doc,
                    Part::MainMatter | Part::Appendix => {
                        after_doc.blocks.push(Block::RawBlock(
                            pandoc_ast::Format("tex".into()),
                            "\\backmatter".into()));
                        part = Part::BackMatter;
                        &mut after_doc
                    }
                    Part::BackMatter => &mut after_doc,
                };
                (doc, &chapter.path, 0)
            }
            mdbook::BookItem::Spacer => continue,
        };
        let item_json = tools::load_file(&src_dir.join(path)).unwrap();
        let mut item_doc = tools::pandoc::from_json(item_json);
        Filter { shift }.walk_pandoc(&mut item_doc);
        doc.meta.extend(item_doc.meta);
        doc.blocks.extend(item_doc.blocks);
    }
    // workaround for a Pandoc quirk https://github.com/jgm/pandoc/issues/855
    before_doc.blocks.push(pandoc_ast::Block::Plain(Default::default()));
    main_doc.blocks.push(pandoc_ast::Block::Plain(Default::default()));
    after_doc.blocks.push(pandoc_ast::Block::Plain(Default::default()));
    (before_doc, main_doc, after_doc)
}

fn main() {
    env_logger::init().unwrap();
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .args_from_usage("<summary-file>")
        .args_from_usage("<out-prefix>")
        .get_matches();
    let summary_file = matches.value_of("summary-file").unwrap();
    let out_prefix = matches.value_of("out-prefix").unwrap();
    let src_dir = Path::new(&summary_file).parent().unwrap();

    let (before_doc, main_doc, after_doc) = merge_book_items(&src_dir);
    // workaround because the default Pandoc template
    // doesn't have a place to insert \frontmatter before \maketitle
    tools::save_file(&Path::new(&(out_prefix.to_owned() + "_head.tex")),
                     "\\AtBeginDocument{\\frontmatter}").unwrap();
    tools::save_file(&Path::new(&(out_prefix.to_owned() + "_before.json")),
                     &tools::pandoc::to_json(&before_doc)).unwrap();
    tools::save_file(&Path::new(&(out_prefix.to_owned() + ".json")),
                     &tools::pandoc::to_json(&main_doc)).unwrap();
    tools::save_file(&Path::new(&(out_prefix.to_owned() + "_after.json")),
                     &tools::pandoc::to_json(&after_doc)).unwrap();
}
