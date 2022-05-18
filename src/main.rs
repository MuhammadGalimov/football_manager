mod engine;
mod player;
mod env;

use std::io::{stdout, Write};
use crossterm::{
    execute,
    terminal::{
        enable_raw_mode,
        Clear,
        ClearType,
        EnterAlternateScreen
    },
    event, style::Print, Result, cursor
};

use engine::*;
use env::Env;

fn main() -> Result<()>{
    let mut stdout = stdout();
    enable_raw_mode()?;

    let mut book =  Book::new(0, build_pages());
    book.draw();

    let mut env = build_env();

    loop {
        execute!(
            &mut stdout,
            Clear(ClearType::All),
            cursor::Hide
        )?;

        execute!(
            &mut stdout,
            Print(&book.draw())
        )?;

        match event::read()? {
            event::Event::Key(event) => {
                match event.code {
                    event::KeyCode::Up => {
                        book.decrease_index();
                    },
                    event::KeyCode::Down => {
                        book.increase_index();
                    },
                    event::KeyCode::Enter => {
                        book.enter(&mut env);
                    },
                    // only for testing
                    event::KeyCode::Char('q') => break,
                    _ => (),
                }
            },
            _ => (),
        }

        stdout.flush()?;
    }

    Ok(())
}

fn build_pages() -> Vec<Page> {
    let mut welcome_page = Page::new(0);
    welcome_page.add_text_widget(
        TextBuilder::new("Добро пожаловать в игру футбольный менеджер!")
        .build()
    );
    welcome_page.add_button_widget(
        ButtonBuilder::new("Новая игра")
        .tagged(true)
        .jump(|_: &mut Env| { Some(1) })
        .build()
    );
    welcome_page.add_button_widget(
        ButtonBuilder::new("Продолжить игру")
        .jump(|_: &mut Env| { Some(2) })
        .build()
    );
    welcome_page.add_button_widget(
        ButtonBuilder::new("Помощь")
        .jump(|_: &mut Env| { Some(3) })
        .build()
    );
    welcome_page.add_button_widget(
        ButtonBuilder::new("Выйти")
        .build()
    );

    let mut new_game_page = Page::new(1);
    new_game_page.add_text_widget(
        TextBuilder::new("Выберите себе команду")
        .build()
    );
    new_game_page.add_button_widget(
        ButtonBuilder::new("Команда номер 1")
            .tagged(true)
            .jump(|_: &mut Env| { None })
            .build()
    );
    new_game_page.add_button_widget(
        ButtonBuilder::new("Команда номер 2")
            .jump(|_: &mut Env| { None })
            .build()
    );
    new_game_page.add_button_widget(
        ButtonBuilder::new("Команда номер 3")
            .jump(|_: &mut Env| { None })
            .build()
    );
    new_game_page.add_button_widget(
        ButtonBuilder::new("Назад")
            .jump(|_: &mut Env| { Some(0) })
            .build()
    );

    let mut continue_page = Page::new(2);
    continue_page.add_text_widget(
        TextBuilder::new("Выберите сохраненную игру")
        .build()
    );
    continue_page.add_button_widget(
        ButtonBuilder::new("Игра 1")
            .tagged(true)
            .jump(|_: &mut Env| { None })
            .build()
    );
    continue_page.add_button_widget(
        ButtonBuilder::new("Назад")
            .jump(|_: &mut Env| { Some(0) })
            .build()
    );

    let mut help_page = Page::new(3);
    help_page.add_text_widget(
        TextBuilder::new("Помощь по игре:")
        .build()
    );
    help_page.add_button_widget(
        ButtonBuilder::new("Назад")
            .tagged(true)
            .jump(|_: &mut Env| { Some(0) })
            .build()
    );

    vec![
        welcome_page, 
        new_game_page, 
        continue_page, 
        help_page
    ]
}

fn build_env() -> Env {
    Env {}
}