use crate::builtins::{BUILTINS, CONSTANTS};
use crate::lexer::{tokenize, TokenKind};
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::{CmdKind, Highlighter};
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};
use std::borrow::Cow;

pub struct RcalHelper;

impl Completer for RcalHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let (start, word) = suffix(line, pos);
        if word.is_empty() {
            return Ok((pos, Vec::new()));
        }

        let mut candidates = Vec::new();
        for b in BUILTINS {
            if b.name.starts_with(word) {
                candidates.push(Pair {
                    display: b.name.to_string(),
                    replacement: b.name.to_string(),
                });
            }
        }
        for (name, _) in CONSTANTS {
            if name.starts_with(word) {
                candidates.push(Pair {
                    display: name.to_string(),
                    replacement: name.to_string(),
                });
            }
        }

        Ok((start, candidates))
    }
}

fn suffix(line: &str, pos: usize) -> (usize, &str) {
    let slice = &line[..pos];
    let pos = slice
        .rfind(|c: char| !c.is_alphanumeric() && c != '_')
        .map_or(0, |i| i + 1);
    (pos, &slice[pos..])
}

impl Hinter for RcalHelper {
    type Hint = String;
}

impl Highlighter for RcalHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        let tokens = match tokenize(line) {
            Ok(t) => t,
            Err(_) => return Cow::Borrowed(line),
        };

        let mut out = String::with_capacity(line.len() * 2);
        let mut last_pos = 0;

        for token in tokens {
            if token.pos > last_pos {
                out.push_str(&line[last_pos..token.pos]);
            }

            if let TokenKind::EOF = token.kind {
                break;
            }

            let color = match &token.kind {
                TokenKind::Identifier(name) => {
                    if BUILTINS.iter().any(|b| b.name == name) {
                        Some("\x1b[34m")
                    } else if CONSTANTS.iter().any(|(n, _)| n == name) {
                        Some("\x1b[33m")
                    } else {
                        Some("\x1b[36m")
                    }
                }
                _ => token.kind.color(),
            };

            if let Some(c) = color {
                out.push_str(c);
                out.push_str(&line[token.pos..token.pos + token.len]);
                out.push_str("\x1b[0m");
            } else {
                out.push_str(&line[token.pos..token.pos + token.len]);
            }
            last_pos = token.pos + token.len;
        }

        if last_pos < line.len() {
            out.push_str(&line[last_pos..]);
        }

        Cow::Owned(out)
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _kind: CmdKind) -> bool {
        true
    }
}

impl Validator for RcalHelper {}
impl Helper for RcalHelper {}
