use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyModifiers},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
};
use std::io::{self};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

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
    let confirmation_row = rows - 3; // Line below the memory output

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
            daemon.memory.get_focus().to_string()
        };

        execute!(
            io::stdout(),
            MoveTo(0, 0),
            Clear(ClearType::CurrentLine),
            Print(format!("THOUGHT: {}", current_thought))
        )
        .unwrap();

        thread::sleep(Duration::from_secs(1));
    });

    let mut input = String::new();
    let mut ctrl_c_pressed_time: Option<Instant> = None;
    let confirmation_window = Duration::from_secs(3); // 3 seconds to confirm exit

    loop {
        execute!(io::stdout(), MoveTo(8, input_row), Print(&input))?;

        if poll(Duration::from_millis(100))? {
            if let Event::Key(event) = read()? {
                match event.code {
                    KeyCode::Enter => {
                        let output = {
                            let mut daemon = daemon.lock().unwrap();
                            daemon.memory.push(input.clone());
                            format!("OUTPUT: {}", daemon.memory.get_response().trim())
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
                        if event.modifiers.contains(KeyModifiers::CONTROL) && c == 'c' {
                            if let Some(last_press) = ctrl_c_pressed_time {
                                if last_press.elapsed() < confirmation_window {
                                    break; // Exit the loop if confirmed within the timeframe
                                }
                            }

                            // Prompt for confirmation
                            ctrl_c_pressed_time = Some(Instant::now());
                            execute!(
                                io::stdout(),
                                MoveTo(0, confirmation_row),
                                Clear(ClearType::CurrentLine),
                                Print("Press again to confirm exit")
                            )?;
                        } else {
                            input.push(c);
                        }
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

        // Reset confirmation if time has passed
        if let Some(last_press) = ctrl_c_pressed_time {
            if last_press.elapsed() >= confirmation_window {
                ctrl_c_pressed_time = None;
                execute!(
                    io::stdout(),
                    MoveTo(0, confirmation_row),
                    Clear(ClearType::CurrentLine)
                )?;
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
