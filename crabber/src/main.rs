// // use std::io::{stdout, Write};
// // use crossterm::{
// //     ExecutableCommand, QueueableCommand,
// //     terminal, cursor, style::{self, Colorize}, Result
// // };

// // fn main() -> Result<()> {
// //   let mut stdout = stdout();

// //   stdout.execute(terminal::Clear(terminal::ClearType::All))?;

// //   for y in 0..40 {
// //     for x in 0..150 {
// //       if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
// //         // in this loop we are more efficient by not flushing the buffer.
// //         stdout
// //           .queue(cursor::MoveTo(x,y))?
// //           .queue(style::PrintStyledContent( "█".blue()))?;
// //       }
// //     }
// //   }
// //   stdout.flush()?;
// //   Ok(())
// // }

// //! Demonstrates how to block read events.
// //!
// //! cargo run --example event-read

// use std::io::{stdout, Write};

// use crossterm::{
//     cursor::position,
//     event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyEvent ,KeyCode, KeyModifiers},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode},
//     Result,
// };

// // const HELP: &str = r#"Blocking read()
// //  - Keyboard, mouse and terminal resize events enabled
// //  - Hit "c" to print current cursor position
// //  - Use Esc to quit
// // "#;

// fn print_events() -> Result<()> {
//     loop {
//         // Blocking read
//         let event = read()?;

//         println!("Event: {:?}\r", event);

//         if event == Event::Key(KeyCode::Char('c').into()) {
//             println!("Cursor position: {:?}\r", position());
//         }

//         if event == Event::Key(KeyEvent {
//             code: KeyCode::Char('c').into(),
//             modifiers: KeyModifiers::CONTROL
//         }) {
//             break;
//         }

//         if event == Event::Key(KeyCode::Esc.into()) {
//             println!("🚀");
//         }
//     }

//     Ok(())
// }

// fn main() -> Result<()> {
//     // println!("{}", HELP);

//     enable_raw_mode()?;

//     let mut stdout = stdout();
//     execute!(stdout, EnableMouseCapture)?;

//     if let Err(e) = print_events() {
//         println!("Error: {:?}\r", e);
//     }

//     execute!(stdout, DisableMouseCapture)?;

//     disable_raw_mode()
// }




// use std::{thread, time};
use std::io::{stdout, Write};
use crossterm::{
    cursor::{DisableBlinking, MoveTo, Hide},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{SetBackgroundColor, Color},
    terminal::{size, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode, Clear, ClearType, SetTitle},
    Result,
};

fn print_events() -> Result<()> {
    let mut stdout = stdout();
    let max_x = size().unwrap().0;
    let max_y = size().unwrap().1;
    let mut player_x = max_x/2;
    let mut player_y = max_y;

    execute!(stdout, Clear(ClearType::All), MoveTo(player_x, player_y-2))?;
    println!("🦀");
    execute!(stdout, MoveTo(player_x, player_y), Clear(ClearType::FromCursorDown))?;
    player_y = player_y-2;
        
    loop {
        let event = read()?;

        if event == Event::Key(KeyEvent {code: KeyCode::Char('c').into(), modifiers: KeyModifiers::CONTROL}) {
            break;
        }

        if event == Event::Key(KeyCode::Char('q').into()) {
            execute!(stdout, Clear(ClearType::FromCursorUp), MoveTo(player_x, player_y))?;
            println!("🦀");
        }

        if event == Event::Key(KeyCode::Char('r').into()) {
            execute!(stdout, Clear(ClearType::FromCursorUp), MoveTo(player_x, player_y))?;
            println!("🚀");
        }
        if event == Event::Key(KeyCode::Char('w').into()) {
            execute!(stdout, Clear(ClearType::FromCursorUp), MoveTo(player_x, player_y))?;
            println!("➤");
        }


        if event == Event::Key(KeyCode::Up.into()) {
            if player_y > 0 {
                player_y = player_y - 1;
            };
            execute!(stdout, Clear(ClearType::FromCursorUp), MoveTo(player_x, player_y))?;
            // println!("ᐃ");
            println!("🦀");
        }

        if event == Event::Key(KeyCode::Down.into()) {
            if player_y < max_y-2 {
                player_y = player_y + 1;
            };
            execute!(stdout, Clear(ClearType::FromCursorUp), MoveTo(player_x, player_y))?;
            // println!("ᐁ");
            println!("🦀");
        }

        // println!("🚲");

        if event == Event::Key(KeyCode::Right.into()) {
            if player_x < max_x-2 {
                player_x = player_x + 1;
            };
            execute!(stdout, Clear(ClearType::FromCursorUp), MoveTo(player_x, player_y))?;
            // println!("ᐅ");
            println!("🦀");
        }

        if event == Event::Key(KeyCode::Left.into()) {
            if player_x > 0 {
                player_x = player_x - 1;
            };
            execute!(stdout, Clear(ClearType::FromCursorUp), MoveTo(player_x, player_y))?;
            // println!("ᐊ");
            println!("🦀");
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(
        stdout, 
        EnterAlternateScreen, 
        DisableBlinking,
        Hide,
        SetTitle("Crabber"), 
        SetBackgroundColor(Color::Rgb{r: 250u8, g: 214u8, b: 76u8})
    )?;

    if let Err(e) = print_events() {
        println!("Error: {:?}\r", e);
    }

    execute!(stdout, LeaveAlternateScreen)?;

    disable_raw_mode()
}

// use std::{
//     thread,
//     time::{Duration, Instant},
// };

// fn main() {
//     let scheduler = thread::spawn(|| {
//         let wait_time = Duration::from_millis(500);

//         // Make this an infinite loop
//         // Or some control path to exit the loop
//         loop {
//             let start = Instant::now();
//             eprintln!("Scheduler starting at {:?}", start);

//             let thread_a = thread::spawn(a);
//             let thread_b = thread::spawn(b);

//             thread_a.join().expect("Thread A panicked");
//             thread_b.join().expect("Thread B panicked");

//             let runtime = start.elapsed();

//             if let Some(remaining) = wait_time.checked_sub(runtime) {
//                 eprintln!(
//                     "schedule slice has time left over; sleeping for {:?}",
//                     remaining
//                 );
//                 thread::sleep(remaining);
//             }
//         }
//     });

//     scheduler.join().expect("Scheduler panicked");
// }

// fn a() {
//     eprintln!("a");
//     thread::sleep(Duration::from_millis(100))
// }
// fn b() {
//     eprintln!("b");
//     thread::sleep(Duration::from_millis(200))
// }