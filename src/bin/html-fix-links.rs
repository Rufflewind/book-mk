extern crate pandoc_ast;
extern crate tools;

use std::io::{self, Read, Write};
use std::collections::BTreeMap;
use std::path::Path;
use pandoc_ast::MutVisitor;

struct AttrVisitor<F>(F);

impl<F: FnMut(&mut pandoc_ast::Attr)> MutVisitor for AttrVisitor<F> {
    fn visit_attr(&mut self, attr: &mut pandoc_ast::Attr) {
        self.0(attr);
    }
}

struct InlineVisitor<F>(F);

impl<F: FnMut(&mut pandoc_ast::Inline)> MutVisitor for InlineVisitor<F> {
    fn visit_inline(&mut self, inline: &mut pandoc_ast::Inline) {
        self.0(inline);
    }
}

fn amalg_iter<F>(amalg: &mut pandoc_ast::Pandoc, mut f: F)
    where F: FnMut(&str, &mut pandoc_ast::Block)
{
    let mut docname = None;
    for block in &mut amalg.blocks {
        match *block {
            pandoc_ast::Block::RawBlock(pandoc_ast::Format(ref fmt), ref s)
                if fmt == "html" =>
            {
                if let Some(n) = tools::match_amalg_prefix(&s) {
                    docname = Some(n.to_owned());
                    continue;
                }
            }
            _ => {}
        }
        f(docname.as_ref().expect("invalid amalg file"), block);
    }
}

fn fix_link(id_to_docname: &BTreeMap<String, String>,
            docname: &str,
            inline: &mut pandoc_ast::Inline) {
    match *inline {
        pandoc_ast::Inline::Link(_, _, (ref mut url, _))
            if url.starts_with("#") =>
        {
            if let Some(target_docname) =
                id_to_docname.get(&url[1 ..])
            {
                let curdir = Path::new(&docname).parent().unwrap();
                if target_docname.as_str() != docname {
                    *url = String::from(
                        Path::new(target_docname)
                            .strip_prefix(curdir).unwrap()
                            .to_str().unwrap())
                        + url;
                }
            }
        }
        _ => {}
    }
}

fn main() {
    let mut json = String::default();
    io::stdin().read_to_string(&mut json).unwrap();
    io::stdout().write_all(&pandoc_ast::filter(json, |mut amalg| {
        let mut id_to_docname = BTreeMap::default();
        amalg_iter(&mut amalg, |docname, block| {
            AttrVisitor(|attr: &mut pandoc_ast::Attr| {
                let id = &attr.0;
                if !id.is_empty() {
                    id_to_docname.insert(id.clone(), docname.to_owned());
                }
            }).walk_block(block);
        });
        amalg_iter(&mut amalg, |docname, block| {
            InlineVisitor(|inline: &mut pandoc_ast::Inline| {
                match *inline {
                    pandoc_ast::Inline::Cite(_, ref mut inlines) => {
                        for inline in inlines {
                            fix_link(&id_to_docname, docname, inline);
                        }
                    }
                    _ => {
                        fix_link(&id_to_docname, docname, inline);
                    }
                }
            }).walk_block(block);
        });
        amalg
    }).as_bytes()).unwrap();
}
