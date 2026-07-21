use reedline::{Reedline, Vi, Prompt, PromptEditMode, DefaultCompleter, FileBackedHistory, MenuBuilder, ColumnarMenu, ReedlineMenu};
use std::borrow::Cow;
use std::path::PathBuf;
use std::env;
mod completer;

pub struct AfshPrompt;

impl Prompt for AfshPrompt {
    fn render_prompt_left(&self) -> Cow<'_,str> {
        let path = match env::current_dir() {
            Ok(a) => a,
            Err(_) => PathBuf::from("/"),
        };
        let path_str = path.to_string_lossy();
        let dis_path = match env::var("HOME") {
            Ok(a) if path_str.starts_with(&a) => path_str.replacen(&a,"~",1),
            _ => path_str.into_owned(),
        };
        Cow::Owned(format!("\x1b[1;38;2;50;130;224m❱❱{}\x1b[0m", dis_path))
    }
    fn render_prompt_right(&self) -> Cow<'_, str> {
        Cow::Borrowed("") 
    }
    fn render_prompt_indicator(&self, mode: PromptEditMode) -> Cow<'_,str> {
       match mode {
           PromptEditMode::Vi(reedline::PromptViMode::Normal) => Cow::Borrowed("$ "),
           PromptEditMode::Vi(reedline::PromptViMode::Insert) => Cow::Borrowed("^$ "),
           _ => Cow::Borrowed("$")
       } 
    }
    fn render_prompt_multiline_indicator(&self) -> Cow<'_,str> {
        Cow::Borrowed("> ")
    }
    fn render_prompt_history_search_indicator(&self, _hist_search: reedline::PromptHistorySearch) -> Cow<'_, str> {
        Cow::Borrowed("? ")
    }
}

pub fn build_rl() -> Reedline {
    let completer = Box::new(completer::AfshCompleter::new());
    let completion_menu = Box::new(ColumnarMenu::default().with_name("completion_menu"));
    let histf = dirs::home_dir().unwrap_or(PathBuf::from("~")).join(".afsh_history");
    let history = Box::new(FileBackedHistory::with_file(10000,histf).expect("afsh: history war gai!"));
    let edtmd = Box::new(Vi::default());

    Reedline::create()
        .with_history(history)
        .with_edit_mode(edtmd)
        .with_completer(completer)
        .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
}

