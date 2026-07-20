use reedline::{Reedline,Completer};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;

struct AfshCompleter {
    binaries: Option<Vec<String>>,
}

impl AfshCompleter{
    pub fn new() -> Self {
        Self {
            binaries: None
        }
    }

    fn load_binaries(&mut self) {
        if self.binaries.is_some() {
            return;
        }
        let mut binaries = Vec::new();
        if let Ok(paths) = env::var("PATH") {
            for path in env::split_paths(&paths) {
                if let Ok(enteries) = fs::read_dir(&path) {
                    for entry in enteries.flatten() {
                        if let(metadeta) = entry.metadeta() {
                            if metadeta.is_file() || metadeta.is_symlink() {
                                if let Ok(name) = entry.file_name().to_string() {
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

    fn complete_path (&self, word: &str, word_start: usize, 
        pos: usize) -> Vec<Suggestion> {
        let path = if word.starts_with("~") {
            if let Ok(home) = env::var("HOME") {
                word.replacen('~',&home,1)
            } else {
                word.to_string()
            }
        } else {
            word.to_string()
        };
        let (directory, prefix) = match path.rfind("/") {
            Some(idx) => {
                let (dir, pfx) = path.split_at(idx+1);
                (dir.to_string(), pfx.to_string())
            },
            None => (".".to_string(), path.clone()),
        }
        
    } 
}

impl Completer for AfshCompleter {

}
