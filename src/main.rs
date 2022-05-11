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
use engine::*;

fn main() -> Result<()>{
    let mut stdout = stdout();
    enable_raw_mode()?;

    // let mut glob = init();

    let mut page = Page::new(0);
    page.add_text_widget(TextBuilder::new("Football Manager").build());
    page.add_button_widget(ButtonBuilder::new("New game").tagged(true).build());
    page.add_button_widget(ButtonBuilder::new("Continue").build());
    page.add_button_widget(ButtonBuilder::new("Help").build());
    page.add_button_widget(ButtonBuilder::new("Exit").build());

    page.draw();

    loop {
        execute!(
            &mut stdout,
            Clear(ClearType::All),
            cursor::Hide
        )?;

        // execute!(
        //     &mut stdout,
        //     Print(&glob.view())
        // )?;

        execute!(
            &mut stdout,
            Print(&page.draw())
        )?;

        match event::read()? {
            event::Event::Key(event) => {
                match event.code {
                    event::KeyCode::Up => {
                        // glob.decrease_index();
                        page.decrease_index();
                    },
                    event::KeyCode::Down => {
                        // glob.increase_index();
                        page.increase_index();
                    },
                    event::KeyCode::Left => {
                        // glob.back();
                    },
                    event::KeyCode::Right => {
                        // glob.enter();
                    },
                    event::KeyCode::Enter => {
                        // glob.enter();
                    },
                    event::KeyCode::Backspace => {
                        // glob.back();
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