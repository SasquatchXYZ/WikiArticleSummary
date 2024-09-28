use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{Clear, ClearType},
};
use serde::Deserialize;
use reqwest::blocking::get;
use std::io::stdout;

#[derive(Debug, Deserialize, Default)]
struct App {
    user_input: String,
}

#[derive(Debug, Deserialize, Default)]
struct CurrentArticle {
    title: String,
    description: String,
    extract: String,
}

const URL: &str = "https://en.wikipedia.org/api/rest_v1/page/summary";

fn main() {
    let mut app = App::default();

    loop {
        if let Event::Key(key_event) = read().unwrap() {
            if key_event.kind == KeyEventKind::Press {
                execute!(stdout(), Clear(ClearType::All)).unwrap();
                match key_event.code {
                    KeyCode::Backspace => {
                        app.user_input.pop();
                        println!("{}", app.user_input);
                    }
                    KeyCode::Esc => app.user_input.clear(),
                    KeyCode::Enter => {
                        print!("Searching Wikipedia...");
                        let req = get(format!("{URL}/{}", app.user_input)).unwrap();
                        let text = req.text().unwrap();
                        let as_article: CurrentArticle = serde_json::from_str(&text).unwrap();
                        println!("{as_article:#?}");
                    }
                    KeyCode::Char(c) => {
                        app.user_input.push(c);
                        println!("{}", app.user_input);
                    }
                    _ => {}
                }
            }
        }
    }
}
