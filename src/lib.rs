//! Pandoc LaTeX acronym filter
//! 
//! This filter translates certain codes in plain markdown text (such as `(+pod)`)
//! into a LaTeX acronym use (`\ac{pod}`). The process is independent from the
//! actual acronym vocabulary, but the full document should provide it for the
//! final compilation to work as intended.
//! 
//! If you wish to use it as a typical pandoc filter, the provided binary is
//! compliant with Pandoc's expected filter inputs and outputs.
//! When constructing complex processing pipelines programmatically,
//! [`make_acronym_formatting`] performs the intended translation in Rust.
//! 
//! [`make_acronym_formatting`]: fn.make_acronym_formatting.html
#![warn(rust_2018_idioms)]

use pandoc_ast::{self, Block, Format, Inline, Pandoc};
use regex::Regex;
use lazy_static::lazy_static;

/// Converts the given pandoc document to expand all occurrences
/// of acronym codes into LaTeX acronym codes.
pub fn make_acronym_formatting(mut pandoc: Pandoc) -> Pandoc {
    for block in &mut pandoc.blocks {
        process_block(block);
    }

    pandoc
}

fn process_block(block: &mut Block) {
    match block {
        // things containing inlines: process each one
        Block::Plain(inlines) | Block::Para(inlines) | Block::Header(_, _, inlines) => {
            process_inlines(inlines);
        }
        // things containing blocks: recursively process those
        Block::BlockQuote(blocks) | Block::Div(_, blocks) => {
            blocks.into_iter().for_each(process_block);
        }
        // this one contains a vector of vectors of inlines
        Block::LineBlock(inline_vectors) => {
            inline_vectors.into_iter().for_each(process_inlines);
        }
        // these ones contain a vector of vectors of blocks
        Block::BulletList(block_vectors) | Block::OrderedList(_, block_vectors) => {
            block_vectors
                .into_iter()
                .flat_map(|i| i)
                .for_each(process_block);
        }
        // this one contains both inlines and blocks
        Block::DefinitionList(list) => {
            for (inlines, blocks) in list {
                process_inlines(inlines);
                blocks.into_iter().flat_map(|i| i).for_each(process_block);
            }
        }
        // table. you special thing you
        Block::Table(inlines, _alignment, _relative_col_widths, header_cells, body_cells) => {
            process_inlines(inlines);
            Iterator::chain(
                header_cells.into_iter().flat_map(|i| i),
                body_cells.into_iter().flat_map(|i| i).flat_map(|i| i),
            ).for_each(process_block);
        }
        // do nothing otherwise
        Block::RawBlock(..) | Block::CodeBlock(..) | Block::HorizontalRule | Block::Null => {}
    }
}

fn process_inlines(inlines: &mut Vec<Inline>) {
    // keep track of changes to make
    let mut changes: Vec<(usize, Inline, Option<String>, Option<String>)> = Vec::new();

    lazy_static! {
        static ref RE: Regex = Regex::new(r"\(\+(\-|\.|\~)?(\*|\^)?(\w)+\)").unwrap();
    }

    for (offset, inline) in inlines.into_iter().enumerate() {
        match inline {
            // recursively address inlines
            Inline::Emph(inlines)
            | Inline::Strong(inlines)
            | Inline::Strikeout(inlines)
            | Inline::Superscript(inlines)
            | Inline::Subscript(inlines)
            | Inline::SmallCaps(inlines)
            | Inline::Quoted(_, inlines)
            | Inline::Link(_, inlines, _)
            | Inline::Image(_, inlines, _)
            | Inline::Span(_, inlines) => process_inlines(inlines),
            // recursively address block
            Inline::Note(blocks) => blocks.into_iter().for_each(process_block),

            // now for the real thing
            Inline::Str(string) => {
                // only one acronym per inline, please
                if let Some(cap) = RE.find(string) {
                    let slice = cap.as_str();
                    let (ty, keyword_offset) = match &slice.as_bytes()[2..4] {
                        b"-*" | b"-^" => (r"\aclp", 4),
                        b".*" | b".^" => (r"\acsp", 4),
                        b"~*" | b"~^" => (r"\acfp", 4),
                        [b'-', _] => (r"\acl", 3),
                        [b'.', _] => (r"\acs", 3),
                        [b'~', _] => (r"\acf", 3),
                        [b'*', _] | [b'^', _] => (r"\acp", 3),
                        _ => (r"\ac", 2),
                    };
                    let keyword = &string[cap.start() + keyword_offset..cap.end() - 1];
                    let raw_inline = Inline::RawInline(
                        Format("tex".to_string()),
                        format!("{}{{{}}}", ty, keyword),
                    );
                    let before = if cap.start() == 0 {
                        None
                    } else {
                        Some(string[..cap.start()].to_string())
                    };
                    let after = if cap.end() == string.len() {
                        None
                    } else {
                        Some(string[cap.end()..].to_string())
                    };
                    changes.push((offset, raw_inline, before, after));
                }
            }
            _ => {}
        }
    }

    let mut offset = 0;
    for (i, command, before, after) in changes {
        // adjust position of insertion (since previous acronym expansion may have yielded more inlines)
        let i = i + offset;

        // replace the whole inline with the acronym command
        inlines[i] = command; 

        // then, insert the extremeties if they exist
        if let Some(s) = after {
            inlines.insert(i + 1, Inline::Str(s));
            offset += 1;
        }

        if let Some(s) = before {
            inlines.insert(i, Inline::Str(s));
            offset += 1;
        }
    }
}
