use nu_ansi_term::{Color, Style};
use parsaf::{error::ParsafError, Lexer, Parser};
use reedline::{Highlighter, StyledText};
use std::{collections::HashSet, ops::Range};

pub struct AfshHighlighter;

fn boundary(ch: char) -> bool {
    ch.is_whitespace() || matches!(ch, '\'' | '"' | '#' | '{' | '}' | '[' | ']' | ';' | ',' | '&' | '|' | '=' | '!' | '<' | '>' | '$')
}

fn push(out: &mut StyledText, style: Style, text: &str) {
    if !text.is_empty() {
        out.push((style, text.to_owned()));
    }
}

fn error_style() -> Style {
    Color::White.on(Color::Red)
}

fn string(out: &mut StyledText, text: &str, quote: char) {
    let style = Color::Green.normal();
    if quote == '\'' {
        push(out, style, text);
        return;
    }
    let end = if text.len() > 1 && text.ends_with('"') { text.len() - 1 } else { text.len() };
    let mut pos = 1;
    let mut plain_start = 0;
    while let Some(offset) = text[pos..end].find('{') {
        let start = pos + offset;
        push(out, style, &text[plain_start..start]);
        pos = text[start..end].find('}').map_or(end, |offset| start + offset + 1);
        push(out, Color::Cyan.normal(), &text[start..pos]);
        plain_start = pos;
    }
    push(out, style, &text[plain_start..]);
}

fn syntax_error(line: &str, fallback: Option<Range<usize>>) -> Option<Range<usize>> {
    let tokens = Lexer::new(line).tokenize().ok()?;
    let error = Parser::new(&tokens).parse().err()?;
    let span = match error {
        ParsafError::NotAllowedHere { span, .. }
        | ParsafError::BaseCase { span, .. }
        | ParsafError::UnexpectedToken { span, .. }
        | ParsafError::ExpectedFound { span, .. } => span,
        ParsafError::UnclosedDelimiter { .. } => return None,
        ParsafError::UnexpectedEof => return fallback,
    };
    if span.len() == 0 || span.offset() >= line.len() {
        fallback
    } else {
        Some(span.offset()..(span.offset() + span.len()).min(line.len()))
    }
}

fn mark_errors(text: StyledText, errors: &[Range<usize>]) -> StyledText {
    let mut result = StyledText::new();
    let mut offset = 0;
    for (style, chunk) in text.buffer {
        let mut start = 0;
        let mut current = style;
        for (index, ch) in chunk.char_indices() {
            let position = offset + index;
            let bad = errors.iter().any(|range| range.start < position + ch.len_utf8() && position < range.end);
            let next = if bad { error_style() } else { style };
            if next != current {
                push(&mut result, current, &chunk[start..index]);
                start = index;
                current = next;
            }
        }
        push(&mut result, current, &chunk[start..]);
        offset += chunk.len();
    }
    result
}

impl Highlighter for AfshHighlighter {
    fn highlight(&self, line: &str, _cursor: usize) -> StyledText {
        let mut out = StyledText::new();
        let mut pos = 0;
        let mut command_start = true;
        let mut declaration_name = false;
        let mut declared = HashSet::new();
        let mut brackets = Vec::new();
        let mut errors = Vec::new();
        let mut statement_head = None;
        let mut fallback = None;
        let mut expect_command = None;
        let mut redirect_target = false;

        while pos < line.len() {
            let start = pos;
            let ch = line[pos..].chars().next().unwrap();
            pos += ch.len_utf8();
            if ch.is_whitespace() {
                if ch == '\n' {
                    command_start = !matches!(brackets.last(), Some(&('[', _)));
                    statement_head = None;
                }
                push(&mut out, Style::new(), &line[start..pos]);
                continue;
            }
            if ch == '#' {
                pos = line[start..].find('\n').map_or(line.len(), |offset| start + offset);
                push(&mut out, Color::Fixed(244).normal(), &line[start..pos]);
                continue;
            }
            if ch == '\'' || ch == '"' {
                if let Some(offset) = line[pos..].find(ch) {
                    pos += offset + ch.len_utf8();
                } else {
                    pos = line.len();
                    errors.push(start..start + ch.len_utf8());
                }
                if ch == '"' {
                    let end = if pos > start + 1 && line[..pos].ends_with('"') { pos - 1 } else { pos };
                    let mut inside = start + 1;
                    while let Some(offset) = line[inside..end].find('{') {
                        let open = inside + offset;
                        match line[open..end].find('}') {
                            Some(offset) => inside = open + offset + 1,
                            None => { errors.push(open..open + 1); break; }
                        }
                    }
                }
                string(&mut out, &line[start..pos], ch);
                if !redirect_target { command_start = false; expect_command = None; }
                redirect_target = false;
            } else if (ch == 'o' || ch == 'e') && line[pos..].starts_with('>') {
                pos += 1;
                if line[pos..].starts_with('>') { pos += 1; }
                push(&mut out, Color::Blue.normal(), &line[start..pos]);
                redirect_target = true;
            } else if boundary(ch) {
                if matches!(ch, '&' | '|' | '=' | '!' | '>') {
                    let next = if ch == '!' { '=' } else { ch };
                    if line[pos..].starts_with(next) { pos += next.len_utf8(); }
                }
                match ch {
                    '{' | '[' => brackets.push((ch, start)),
                    '}' | ']' => {
                        let expected = if ch == '}' { '{' } else { '[' };
                        match brackets.last() {
                            Some(&(open, _)) if open == expected => { brackets.pop(); }
                            _ => errors.push(start..pos),
                        }
                    }
                    _ => {},
                }
                push(&mut out, Color::Blue.normal(), &line[start..pos]);
                if matches!(ch, '{' | ';' | '|' | '&') {
                    command_start = true;
                    statement_head = None;
                    if ch == '|' || &line[start..pos] == "&&" {
                        expect_command = Some(start..pos);
                    }
                }
                if ch == '[' { command_start = false; }
                if matches!(ch, '<' | '>') { redirect_target = true; }
            } else {
                while let Some(next) = line[pos..].chars().next() {
                    if boundary(next) { break; }
                    pos += next.len_utf8();
                }
                let word = &line[start..pos];
                if statement_head.is_none() {
                    statement_head = Some(start..pos);
                }
                fallback = statement_head.clone();
                let style = if redirect_target {
                    redirect_target = false;
                    Color::Green.normal()
                } else if declaration_name {
                    declaration_name = false;
                    command_start = false;
                    declared.insert(word.to_owned());
                    Color::Cyan.normal()
                } else {
                    expect_command = None;
                    match word {
                        "fn" | "alias" => {
                            declaration_name = true;
                            command_start = false;
                            Color::Cyan.normal()
                        }
                        "if" | "elif" | "while" => {
                            command_start = true;
                            Color::Cyan.normal()
                        }
                        "let" | "export" | "source" | "print" | "else" |
                        "for" | "in" | "to" | "break" | "true" | "false" => {
                            command_start = false;
                            Color::Cyan.normal()
                        }
                        _ if word.parse::<i64>().is_ok() || word.parse::<f64>().is_ok() => {
                            command_start = false;
                            Color::Green.normal()
                        }
                        _ if command_start => {
                            command_start = false;
                            Color::Cyan.normal()
                        }
                        _ => Color::Green.normal(),
                    }
                };
                push(&mut out, style, word);
            }
        }
        errors.extend(brackets.into_iter().map(|(_, index)| index..index + 1));
        if let Some(range) = expect_command { errors.push(range); }
        if let Some(range) = syntax_error(line, fallback) { errors.push(range); }
        mark_errors(out, &errors)
    }
}

