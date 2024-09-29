use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{Clear, ClearType},
};
use serde::{Deserialize, Serialize};
use reqwest::blocking::get;
use std::{error::Error, io::stdout};
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize, Default)]
struct CurrentArticle {
    title: String,
    description: String,
    extract: String,
}

#[derive(Debug, Default)]
struct App {
    current_article: CurrentArticle,
    search_string: String,
}

impl App {
    fn get_article(&mut self) -> Result<(), Box<dyn Error>> {
        let text = get(format!("{URL}/{}", self.search_string))?.text()?;
        if let Ok(article) = serde_json::from_str::<CurrentArticle>(&text) {
            self.current_article = article;
        }
        Ok(())
    }
}

impl std::fmt::Display for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
                    Searching for: {}
Title: {}
------------
Description: {}
------------
{}",
            self.search_string,
            self.current_article.title,
            self.current_article.description,
            self.current_article.extract
        )
    }
}

const URL: &str = "https://en.wikipedia.org/api/rest_v1/page/summary";

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::default();

    loop {
        println!("{app}");
        if let Event::Key(key_event) = read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Backspace => {
                        app.search_string.pop();
                    }
                    KeyCode::Esc => app.search_string.clear(),
                    KeyCode::Enter => app.get_article()?,
                    KeyCode::Char(c) => {
                        app.search_string.push(c);
                    }
                    _ => {}
                }
            }
            execute!(stdout(), Clear(ClearType::All))?;
        }
    }
}
