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

mod daemon;
mod genome;
mod memory;
use daemon::Cortex;

fn main() -> io::Result<()> {
    let _seq1: Vec<f64> = genome::generate_sequence(23);
    let _seq2: Vec<f64> = genome::generate_sequence(23);

    let daemon = Arc::new(Mutex::new(Cortex::new()));
    let daemon_clone = Arc::clone(&daemon);

    enable_raw_mode()?;
    let (_, rows) = size()?;
    let input_row = rows - 1;
    let output_row = rows - 2;

    execute!(
        io::stdout(),
        Clear(ClearType::All),
        Hide,
        MoveTo(0, 0),
        Print("Current thought: "),
        MoveTo(0, input_row),
        Print(" INPUT: ")
    )?;

    // Spawn a thread to update and display the current thought
    thread::spawn(move || loop {
        let current_thought = {
            let daemon = daemon_clone.lock().unwrap();
            daemon.memory.get_current().to_string()
        };

        execute!(
            io::stdout(),
            MoveTo(0, 0),
            Clear(ClearType::CurrentLine),
            Print(format!("Current thought: {}", current_thought))
        )
        .unwrap();

        thread::sleep(Duration::from_secs(1));
    });

    let mut input = String::new();
    let mut output = String::new();

    loop {
        execute!(io::stdout(), MoveTo(8, input_row), Print(&input))?; // Moved cursor position by 1

        if poll(Duration::from_millis(100))? {
            if let Event::Key(event) = read()? {
                match event.code {
                    KeyCode::Enter => {
                        output = format!("OUTPUT: {}", input.trim());
                        execute!(
                            io::stdout(),
                            MoveTo(0, output_row),
                            Clear(ClearType::CurrentLine),
                            Print(&output)
                        )?;

                        {
                            let mut daemon = daemon.lock().unwrap();
                            daemon.memory.push(input.clone());
                        }

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
            Print(" INPUT: "), // Note the leading space here
            Print(&input)
        )?;
    }

    disable_raw_mode()?;
    execute!(io::stdout(), Show)?;

    Ok(())
}
