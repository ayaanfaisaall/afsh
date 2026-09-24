use reedline::{
    Reedline, Vi, Prompt, PromptEditMode, FileBackedHistory, 
    MenuBuilder, ColumnarMenu, ReedlineMenu, KeyCode, KeyModifiers, ReedlineEvent, 
    default_vi_insert_keybindings, default_vi_normal_keybindings };
use std:: { env, fs, borrow::Cow, path::{ Path, PathBuf } , process::Command };
use chrono::Local;
mod completer;

pub struct AfshPrompt;

const GREY_BG: &str = "48;5;236";
const GREY_FG: &str = "38;5;236";

const LIGHT_BLUE_BG: &str = "48;2;50;130;224"; 
const LIGHT_BLUE_FG: &str = "38;2;50;130;224";

const DARK_BLUE_BG: &str = "48;2;25;80;155";
const DARK_BLUE_FG: &str = "38;2;25;80;155";

const DARKEST_BLUE_BG: &str = "48;2;15;45;95";
const DARKEST_BLUE_FG: &str = "38;2;15;45;95";

const GREEN_BG: &str = "48;2;45;125;65";
const GREEN_FG: &str = "38;2;45;125;65";

const RED_BG: &str = "48;2;160;50;50";
const RED_FG: &str = "38;2;160;50;50";

fn get_git_branch() -> Option<String> {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .ok()?;

    if output.status.success() {
        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !branch.is_empty() {
            return Some(branch);
        }
    }
    None
}

fn get_battery_info() -> Option<(u8, bool)> {
    let base = Path::new("/sys/class/power_supply");
    
    // Most laptops use BAT0 or BAT1
    let bat_path = if base.join("BAT0").exists() {
        base.join("BAT0")
    } else if base.join("BAT1").exists() {
        base.join("BAT1")
    } else {
        return None;
    };

    let capacity = fs::read_to_string(bat_path.join("capacity"))
        .ok()?
        .trim()
        .parse::<u8>()
        .ok()?;

    let is_charging = fs::read_to_string(bat_path.join("status"))
        .ok()?
        .trim() == "Charging";

    Some((capacity, is_charging))
}

impl Prompt for AfshPrompt {

    fn render_prompt_left(&self) -> Cow<'_, str> {
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

        let time_str = Local::now().format("%H:%M").to_string();
        let mut prompt = String::new();

        prompt.push_str(&format!("\x1b[{}m╭", LIGHT_BLUE_FG));

        prompt.push_str(&format!(
            "\x1b[{}m\x1b[38;5;255m    {}  ",
            GREY_BG, display_path
        ));

        prompt.push_str(&format!(
            "\x1b[{}m\x1b[{}m",
            LIGHT_BLUE_BG, GREY_FG
        ));

        if let Some(branch) = get_git_branch() {
            prompt.push_str(&format!(
                "\x1b[{}m\x1b[1;38;5;232m   {}  ",
                LIGHT_BLUE_BG, branch
            ));
        }

        let battery_info = get_battery_info();
        
        let mut bat_bg = DARK_BLUE_BG; 
        let mut bat_fg = DARK_BLUE_FG;
        let mut bat_text = String::new();

        if let Some((capacity, charging)) = battery_info {
            if charging {
                bat_bg = GREEN_BG;
                bat_fg = GREEN_FG;
                bat_text = format!("  {}%  ", capacity);
            } else if capacity < 20 {
                bat_bg = RED_BG;
                bat_fg = RED_FG;
                bat_text = format!("  {}%  ", capacity);
            } else {
                bat_text = "".to_string(); 
            }
        }

        // Triangle 2: Git -> Battery
        prompt.push_str(&format!(
            "\x1b[{}m\x1b[{}m",
            bat_bg, LIGHT_BLUE_FG
        ));

        // 4. BATTERY SEGMENT (Always uses white text)
        if !bat_text.is_empty() {
            prompt.push_str(&format!(
                "\x1b[{}m\x1b[38;5;255m{}",
                bat_bg, bat_text
            ));
        }

        // Triangle 3: Battery -> Time (Darkest Blue)
        prompt.push_str(&format!(
            "\x1b[{}m\x1b[{}m",
            DARKEST_BLUE_BG, bat_fg
        ));

        // 5. TIME SEGMENT (Now Darkest Blue, switched to white text for readability)
        prompt.push_str(&format!(
            "\x1b[{}m\x1b[38;5;255m  󰥔 {} ",
            DARKEST_BLUE_BG, time_str
        ));

        // 6. END TRIANGLE (Darkest Blue)
        prompt.push_str(&format!(
            "\x1b[0m\x1b[{}m\x1b[0m\n",
            DARKEST_BLUE_FG
        ));

        // let formatted = format!(
        //     "\x1b[38;2;50;130;224m┌\x1b[48;5;236m\x1b[38;5;255m    {}  \
        //      \x1b[48;2;50;130;224m\x1b[38;5;236m\
        //      \x1b[48;2;50;130;224m\x1b[1;38;5;232m  󰥔 {} \
        //      \x1b[0m\x1b[38;2;50;130;224m\x1b[0m\n",
        //     display_path, time_str
        // );

        Cow::Owned(prompt)
    }

    fn render_prompt_right(&self) -> Cow<'_, str> {
        Cow::Borrowed("") 
    }

    fn render_prompt_indicator(&self, mode: PromptEditMode) -> Cow<'_, str> {
        match mode {
            PromptEditMode::Vi(reedline::PromptViMode::Normal) => Cow::Borrowed("\x1b[1;38;2;50;130;224m╰──$\x1b[0m "),
            PromptEditMode::Vi(reedline::PromptViMode::Insert) => Cow::Borrowed("\x1b[1;38;2;50;130;224m╰──$❱\x1b[0m "),
            _ => Cow::Borrowed("\x1b[1;32m❯\x1b[0m ")
        } 
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<'_, str> {
        Cow::Borrowed("\x1b[1;32m>\x1b[0m ")
    }

    fn render_prompt_history_search_indicator(&self, _hist_search: reedline::PromptHistorySearch) -> Cow<'_, str> {
        Cow::Borrowed("\x1b[1;38;2;50;130;224m? ❯\x1b[0m ")
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

