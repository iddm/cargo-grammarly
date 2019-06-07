//! The docs module contains all the necessary stuff to work with doc comments.

use std::collections::HashMap;

// Ideally error should be printed that way:
//
// error[grammar]: Missing subject
// --> src/main.rs:138:16
//     |
// 138 | /// Reads the .rs files in the directory recursively.
//     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//     |     - This sentence appears to be missing a subject. Consider adding a subject or rewriting the sentence.
//
//
//
// error[grammar]: Spelling
// --> src/main.rs:138:16
//     |
// 138 | /// Thisf module is for easing the pain with printing text in the terminal.
//     |     ^^^^^
//     |     - The word "Thisf" is not in our dictionary. If you are sure this spelling is correct,
//     |     - you can add it to your personal dictionary to prevent future alerts.
//

#[derive(Debug, Clone)]
pub struct Docs(pub HashMap<String, Vec<proc_macro2::Literal>>);

impl Docs {
    fn append(&mut self, docs: Docs) {
        for (k, mut v) in docs.0 {
            self.0.entry(k).or_insert_with(Vec::new).append(&mut v);
        }
    }
}

impl<T> From<(T, proc_macro2::TokenStream)> for Docs
where
    T: AsRef<str>,
{
    fn from(stream: (T, proc_macro2::TokenStream)) -> Docs {
        use proc_macro2::TokenTree;

        let mut docs = Docs(HashMap::new());
        let mut is_doc = false;
        for tree in stream.1 {
            match tree {
                TokenTree::Ident(ident) => is_doc = ident == "doc",
                TokenTree::Group(group) => {
                    docs.append(Docs::from((stream.0.as_ref().to_owned(), group.stream())))
                }
                TokenTree::Literal(literal) => {
                    if is_doc {
                        docs.0
                            .entry(stream.0.as_ref().to_owned())
                            .or_insert_with(Vec::new)
                            .push(literal);
                    }
                }
                _ => {}
            };
        }
        docs
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FixedDocPos {
    pub line: usize,
    pub column: usize,
}

impl From<proc_macro2::LineColumn> for FixedDocPos {
    fn from(span: proc_macro2::LineColumn) -> FixedDocPos {
        FixedDocPos {
            line: span.line,
            column: span.column,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FixedDocSpan {
    pub start: FixedDocPos,
    pub end: FixedDocPos,
}

impl From<proc_macro2::Span> for FixedDocSpan {
    fn from(span: proc_macro2::Span) -> FixedDocSpan {
        FixedDocSpan {
            start: span.start().into(),
            end: span.end().into(),
        }
    }
}

/// The doc is considered "fixed" when it contains text only.
#[derive(Debug, Clone)]
pub struct FixedDoc {
    pub text: String,
    pub span: FixedDocSpan,
    pub check_response: Option<grammarly::Response>,
}

#[derive(Debug, Clone)]
pub struct FixedDocs {
    pub original: Docs,
    pub fixed: HashMap<String, Vec<FixedDoc>>,
    // mappings: Vec<usize>,
}

fn fix_string(s: &str) -> String {
    s.replace("///", "")
        .replace("//!", "")
        .replace(r#"\""#, r#"""#)
        .trim_matches('\"')
        .trim()
        .to_owned()
}

// impl<T> From<T> for FixedDocs where T: AsRef<Docs> {
//     fn from(original: T) -> FixedDocs {
impl From<Docs> for FixedDocs {
    fn from(original: Docs) -> FixedDocs {
        // let mut fixed = Docs(HashMap::new());
        // let mut mappings = Vec::new();
        // let original = original.as_ref();
        let mut fixed: HashMap<String, Vec<FixedDoc>> = HashMap::new();

        for (file, docs) in &original.0 {
            for doc in docs {
                let original_string = doc.to_string();
                let fixed_string = &fix_string(&original_string);
                let start_column_diff = original_string.len() - fixed_string.len();
                let mut span: FixedDocSpan = doc.span().into();
                span.start.column += start_column_diff;
                let current = FixedDoc {
                    text: fixed_string.clone(),
                    span,
                    check_response: None,
                };

                if !fixed.contains_key(file) {
                    fixed.insert(file.clone(), vec![current]);
                } else {
                    let fixed_docs = fixed.get_mut(file).unwrap();
                    let last = fixed_docs.last_mut().unwrap();

                    // If the lines are consecutive, then these two doc comments belong to a single block.
                    if current.span.start.line - last.span.end.line == 1 {
                        last.text.push_str(&format!(" {}", fixed_string));
                        last.span.end = current.span.end;
                    } else {
                        fixed_docs.push(current);
                    }
                }
            }
        }

        FixedDocs {
            original,
            fixed,
            // mappings,
        }
    }
}
