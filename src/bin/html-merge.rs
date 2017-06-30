extern crate clap;
extern crate mdbook;
extern crate pandoc_ast;
extern crate tools;

use std::io::Write;
use std::path::Path;

fn amalg_push(amalg: &mut pandoc_ast::Pandoc,
              docname: &str,
              doc: pandoc_ast::Pandoc) {
    amalg.meta.extend(doc.meta);
    amalg.blocks.push(pandoc_ast::Block::RawBlock(
        pandoc_ast::Format("html".into()),
        format!("<!-- {} {} -->", tools::AMALG_MAGIC, docname),
    ));
    amalg.blocks.extend(doc.blocks);
}

fn main() {
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .args_from_usage("--final-ext=<extension>")
        .args_from_usage("--output-ext=<extension>")
        .args_from_usage("--output-dir=<path>")
        .args_from_usage("--biblio-path=<path>")
        .args_from_usage("<input-dir>")
        .get_matches();
    let final_ext = Path::new(matches.value_of_os("final-ext").unwrap());
    let output_ext = Path::new(matches.value_of_os("output-ext").unwrap());
    let output_dir = Path::new(matches.value_of_os("output-dir").unwrap());
    let biblio_path = Path::new(matches.value_of_os("biblio-path").unwrap());
    let input_dir = Path::new(matches.value_of_os("input-dir").unwrap());

    let mut amalg = tools::pandoc::empty();

    let book_items = tools::mdbook::get_items(&input_dir).unwrap();
    let mut new_summary = String::default();
    for item in tools::mdbook::iter_items(&book_items) {
        let (prefix, chapter) = match *item {
            mdbook::BookItem::Chapter(ref number, ref chapter) => {
                let depth = tools::mdbook::chapter_depth(number);
                let prefix = format!("{}- ", "    ".repeat(depth as _));
                (prefix, chapter)
            }
            mdbook::BookItem::Affix(ref chapter) => {
                (Default::default(), chapter)
            }
            mdbook::BookItem::Spacer => {
                new_summary += "---";
                continue;
            }
        };
        new_summary += &format!(
            "{}[{}]({})\n",
            prefix,
            chapter.name,
            chapter.path.with_extension(output_ext).display(),
        );
        let doc = tools::pandoc::from_json(
            tools::load_file(&input_dir.join(&chapter.path)).unwrap());
        amalg_push(&mut amalg,
                   chapter.path.with_extension(final_ext).to_str().unwrap(),
                   doc);
    }

    // add Bibliography section if needed
    if amalg.meta.contains_key("bibliography") {
        let title = match amalg.meta.get("biblio-title") {
            Some(&pandoc_ast::MetaValue::MetaString(ref title)) => title,
            _ => "Bibliography",
        }.to_owned();
        new_summary += &format!(
            "[{}]({})\n",
            title,
            biblio_path.with_extension(output_ext).display(),
        );
        let mut doc = tools::pandoc::empty();
        doc.blocks.push(pandoc_ast::Block::Header(
            1,
            Default::default(),
            vec![pandoc_ast::Inline::Str(title)],
        ));
        amalg_push(&mut amalg,
                   biblio_path.with_extension(final_ext).to_str().unwrap(),
                   doc);
    }

    tools::save_file(&output_dir.join("SUMMARY.md"),
                     &new_summary).unwrap();
    std::io::stdout().write_all(tools::pandoc::to_json(&amalg)
                                .as_bytes()).unwrap();
}
