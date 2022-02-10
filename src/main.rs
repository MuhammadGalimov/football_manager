mod engine;
mod game;

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
use engine::Glob;

fn main() -> Result<()>{

    let game = game::game::Game::new();
    game.save();

    let mut stdout = stdout();
    enable_raw_mode()?;

    let mut glob = init();

    loop {
        execute!(
            &mut stdout,
            Clear(ClearType::All),
            cursor::Hide
        )?;

        execute!(
            &mut stdout,
            Print(&glob.view())
        )?;

        match event::read()? {
            event::Event::Key(event) => {
                match event.code {
                    event::KeyCode::Up => {
                        glob.decrease_index();
                    },
                    event::KeyCode::Down => {
                        glob.increase_index();
                    },
                    event::KeyCode::Left => {
                        glob.back();
                    },
                    event::KeyCode::Right => {
                        glob.enter();
                    },
                    event::KeyCode::Enter => {
                        glob.enter();
                    },
                    event::KeyCode::Backspace => {
                        glob.back();
                    },
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

fn init() -> Glob {
    let mut glob = Glob::new();
    glob.add_node("root", "new_game", "Новая игра").unwrap();
    glob.add_node("root", "continue", "Продолжить").unwrap();
    glob.add_node("root", "exit", "Выход").unwrap();
    glob.add_node("new_game", "choose_team", "Выбрать команду");
    glob
}
