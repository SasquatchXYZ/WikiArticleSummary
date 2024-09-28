use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{Clear, ClearType},
};
use reqwest::blocking::get;
use std::io::stdout;

#[derive(Debug, Default)]
struct App {
    user_input: String,
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
                        println!("{text}");
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
