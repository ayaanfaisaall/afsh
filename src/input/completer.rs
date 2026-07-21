use reedline::{Completer, Span, Suggestion};
use std::env;
use std::fs;

pub struct AfshCompleter {
    binaries: Option<Vec<String>>,
}

impl AfshCompleter {
    pub fn new() -> Self {
        Self { binaries: None }
    }

    fn load_binaries(&mut self) {
        if self.binaries.is_some() {
            return;
        }
        let mut binaries = Vec::new();
        if let Ok(paths) = env::var("PATH") {
            for path in env::split_paths(&paths) {
                if let Ok(entries) = fs::read_dir(&path) {
                    for entry in entries.flatten() {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_file() || file_type.is_symlink() {
                                if let Ok(name) = entry.file_name().into_string() {
                                    binaries.push(name);
                                }
                            }
                        }
                    }
                }
            }
        }
        binaries.sort();
        binaries.dedup();
        self.binaries = Some(binaries);
    }

    fn complete_path(&self, word: &str, word_start: usize, pos: usize) -> Vec<Suggestion> {
        let path = if word.starts_with("~") {
            if let Ok(home) = env::var("HOME") {
                word.replacen('~', &home, 1)
            } else {
                word.to_string()
            }
        } else {
            word.to_string()
        };

        let (directory, prefix) = match path.rfind("/") {
            Some(idx) => {
                let (dir, pfx) = path.split_at(idx + 1);
                (dir.to_string(), pfx.to_string())
            }
            None => (".".to_string(), path.clone()),
        }; 

        let pfx_line = match word.rfind("/") {
            Some(idx) => word_start + idx + 1,
            None => word_start,
        };
        
        let mut suggestions = Vec::new();
        
        if let Ok(entries) = fs::read_dir(directory) {
            for entry in entries.filter_map(Result::ok) {
                let file = entry.file_name().to_string_lossy().to_string();
                if file.starts_with(&prefix) {
                    let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                    let mut suggestion = file; 
                    let mut append_whitespace = true;

                    if is_dir {
                        suggestion.push('/');
                        append_whitespace = false;
                    }

                    suggestions.push(Suggestion {
                        value: suggestion,
                        description: Some("path".to_string()), 
                        extra: None,
                        span: Span::new(pfx_line, pos),
                        append_whitespace,
                        match_indices: vec![].into(),
                        display_override: None,
                        style: None,
                    });
                }
            }
        }
        suggestions
    }
}

impl Completer for AfshCompleter {
    fn complete(&mut self, line: &str, pos: usize) -> Vec<Suggestion> {

        let line_upto_pfx = &line[..pos];
        let word_start = line_upto_pfx.rfind(' ').map(|i| i + 1).unwrap_or(0);
        let word_to_complete = &line_upto_pfx[word_start..];
        let mut suggestions = Vec::new();

        if word_start == 0 && !word_to_complete.contains('/') {
            self.load_binaries();
            let bins = self.binaries.as_ref().unwrap();
            let start_idx = bins.partition_point(|x| x.as_str() < word_to_complete);

            for bin in &bins[start_idx..] {
                if bin.starts_with(word_to_complete) {
                    suggestions.push(Suggestion {
                        value: bin.clone(),
                        description: Some("command".to_string()),
                        extra: None,
                        span: Span::new(word_start, pos),
                        append_whitespace: true, 
                        match_indices: vec![].into(),
                        display_override: None,
                        style: None,
                    });
                } else {
                    break;
                }
            }
        }

        let mut path_suggestions = self.complete_path(word_to_complete, word_start, pos);
        suggestions.append(&mut path_suggestions);

        if suggestions.len() > 1 {
            let mut prefix = suggestions[0].value.clone();
            
            for sug in &suggestions[1..] {
                let cmn_bytes = prefix
                    .bytes()
                    .zip(sug.value.bytes())
                    .take_while(|(a,b)| a==b)
                    .count();
                prefix.truncate(cmn_bytes);
            }

            if prefix.len() > word_to_complete.len() {
                let span = suggestions[0].span;
                suggestions = vec![Suggestion{
                    value: prefix,
                    description: None,
                    extra: None,
                    span,
                    append_whitespace: false,
                    match_indices: vec![].into(),
                    display_override: None,
                    style: None,
                }];
            } 
        }

        suggestions
    }
}
