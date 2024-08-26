use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
};
use std::io::{self};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod core;
mod memory;
use core::Simulator;

fn main() -> io::Result<()> {
    let daemon = Arc::new(Mutex::new(Simulator::new()));
    let daemon_clone = Arc::clone(&daemon);

    enable_raw_mode()?;
    let (_, rows) = size()?;
    let input_row = rows - 1;
    let output_row = rows - 2;

    execute!(
        io::stdout(),
        Clear(ClearType::All),
        Hide,
        MoveTo(0, input_row),
        Print(" INPUT: ")
    )?;

    // Spawn a thread to update and display the current thought
    thread::spawn(move || loop {
        let current_thought = {
            let daemon = daemon_clone.lock().unwrap();
            daemon.get_focus().to_string()
        };

        execute!(
            io::stdout(),
            MoveTo(0, 0),
            Clear(ClearType::CurrentLine),
            Print(format!("ATTENTION: {}", current_thought))
        )
        .unwrap();

        thread::sleep(Duration::from_secs(1));
    });

    let mut input = String::new();

    loop {
        execute!(io::stdout(), MoveTo(8, input_row), Print(&input))?;

        if poll(Duration::from_millis(100))? {
            if let Event::Key(event) = read()? {
                match event.code {
                    KeyCode::Enter => {
                        let output = {
                            let mut daemon = daemon.lock().unwrap();
                            daemon.memory.push(input.clone());
                            format!("OUTPUT: {}", daemon.get_response().trim())
                        };

                        execute!(
                            io::stdout(),
                            MoveTo(0, output_row),
                            Clear(ClearType::CurrentLine),
                            Print(&output)
                        )?;

                        input.clear();
                    }
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
        }

        execute!(
            io::stdout(),
            MoveTo(0, input_row),
            Clear(ClearType::CurrentLine),
            Print(" INPUT: "),
            Print(&input)
        )?;
    }

    disable_raw_mode()?;
    execute!(io::stdout(), Show)?;

    Ok(())
}
