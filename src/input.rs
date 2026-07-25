use reedline::{
    Reedline, Vi, Prompt, PromptEditMode, FileBackedHistory, 
    MenuBuilder, ColumnarMenu, ReedlineMenu, KeyCode, KeyModifiers, ReedlineEvent, 
    default_vi_insert_keybindings, default_vi_normal_keybindings };
use std:: { env, borrow::Cow, path::PathBuf };
mod completer;

pub struct AfshPrompt;

impl Prompt for AfshPrompt {
    fn render_prompt_left(&self) -> Cow<'_,str> {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));

        let display_path = if let Some(home) = dirs::home_dir() {
            match current_dir.strip_prefix(&home) {
                Ok(stripped) => {
                    let mut p = PathBuf::from("~");
                    p.push(stripped);
                    p.to_string_lossy().into_owned()
                }
                Err(_) => current_dir.to_string_lossy().into_owned(),
            }
        } else {
            current_dir.to_string_lossy().into_owned()
        };

        Cow::Owned(format!("\x1b[1;38;2;50;130;224m❱❱{}\x1b[0m", display_path))
    }
    fn render_prompt_right(&self) -> Cow<'_, str> {
        Cow::Borrowed("") 
    }
    fn render_prompt_indicator(&self, mode: PromptEditMode) -> Cow<'_,str> {
       match mode {
           PromptEditMode::Vi(reedline::PromptViMode::Normal) => Cow::Borrowed("$ "),
           PromptEditMode::Vi(reedline::PromptViMode::Insert) => Cow::Borrowed("^$ "),
           _ => Cow::Borrowed("$ ")
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
    let history = Box::new(FileBackedHistory::with_file(10000,histf).expect("afsh: cannot load history"));
    let mut bindings = default_vi_insert_keybindings();
    
    bindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Tab,
        ReedlineEvent::Menu("completion_menu".to_string()),
    );

    let edtmd = Box::new(Vi::new(bindings, default_vi_normal_keybindings()));

    Reedline::create()
        .with_history(history)
        .with_edit_mode(edtmd)
        .with_completer(completer)
        .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
        .with_quick_completions(true)
}

