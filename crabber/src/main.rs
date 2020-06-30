use crossterm::{
    cursor::{DisableBlinking, Hide, MoveTo},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
        SetTitle,
    },
    Result,
};
use rand::{thread_rng, Rng};
use std::io::{stdout, Write};

fn print_events() -> Result<()> {
    let cactus = "🌵";
    let player = "🦀";
    let bug = "🐛";
    let blank = "  ";

    let mut stdout = stdout();
    let mut rng = thread_rng();
    let max_x: u16 = size().unwrap().0 / 2;
    let max_y: u16 = size().unwrap().1;
    let mut player_x = max_x / 2;
    let mut player_y = max_y / 2;

    let mut grid = vec![vec![0; max_y.into()]; max_x.into()];

    grid[player_x as usize][player_y as usize] = 0;

    grid[rng.gen_range(1, max_x) as usize][rng.gen_range(1, max_y) as usize] = 2;

    for x in 0..max_x {
        for y in 0..max_y {
            if grid[x as usize][y as usize] == 0 {
                queue!(stdout, MoveTo(x * 2, y))
                    .map_err(|err| println!("{:?}", err))
                    .ok();
                queue!(stdout, Print(blank))
                    .map_err(|err| println!("{:?}", err))
                    .ok();
            }
            if grid[x as usize][y as usize] == 1 {
                queue!(stdout, MoveTo(x * 2, y))
                    .map_err(|err| println!("{:?}", err))
                    .ok();
                queue!(stdout, Print(cactus))
                    .map_err(|err| println!("{:?}", err))
                    .ok();
            }
            if x == player_x && y == player_y {
                queue!(stdout, MoveTo(x * 2, y))
                    .map_err(|err| println!("{:?}", err))
                    .ok();
                queue!(stdout, Print(player))
                    .map_err(|err| println!("{:?}", err))
                    .ok();
            }
            if grid[x as usize][y as usize] == 2 {
                queue!(stdout, MoveTo(x * 2, y))
                    .map_err(|err| println!("{:?}", err))
                    .ok();
                queue!(stdout, Print(bug))
                    .map_err(|err| println!("{:?}", err))
                    .ok();
            }
        }
    }
    stdout.flush().map_err(|err| println!("{:?}", err)).ok();

    loop {
        let event = read()?;

        if event
            == Event::Key(KeyEvent {
                code: KeyCode::Char('c').into(),
                modifiers: KeyModifiers::CONTROL,
            })
        {
            break;
        }
        if event == Event::Key(KeyCode::Up.into()) {
            if player_y > 0 {
                player_y = player_y - 1;
                execute!(stdout, MoveTo(player_x * 2, player_y + 1))?;
                println!("  ");
                execute!(stdout, MoveTo(player_x * 2, player_y))?;
                println!("🦀");
                if grid[player_x as usize][player_y as usize] == 2 {
                    // grid[player_x as usize][player_y as usize] = 0;
                    let mut i_x = rng.gen_range(0, max_x);
                    let mut i_y = rng.gen_range(0, max_y-1);

                    while i_x == player_x {
                        i_x = rng.gen_range(0, max_x);
                    }
                    while i_y == player_y {
                        i_y = rng.gen_range(0, max_y-1);
                    }
                    grid[i_x as usize][i_y as usize] = 2;
                    execute!(stdout, MoveTo(i_x * 2, i_y))?;
                    println!("🐛");
                }
            };
        }
        if event == Event::Key(KeyCode::Down.into()) {
            if player_y < max_y - 2 {
                player_y = player_y + 1;
                execute!(stdout, MoveTo(player_x * 2, player_y - 1))?;
                println!("  ");
                execute!(stdout, MoveTo(player_x * 2, player_y))?;
                println!("🦀");
                if grid[player_x as usize][player_y as usize] == 2 {
                    // grid[player_x as usize][player_y as usize] = 0;
                    let mut i_x = rng.gen_range(0, max_x);
                    let mut i_y = rng.gen_range(0, max_y-1);

                    while i_x == player_x {
                        i_x = rng.gen_range(0, max_x);
                    }
                    while i_y == player_y {
                        i_y = rng.gen_range(0, max_y-1);
                    }
                    grid[i_x as usize][i_y as usize] = 2;
                    execute!(stdout, MoveTo(i_x * 2, i_y))?;
                    println!("🐛");
                }
            };
        }
        if event == Event::Key(KeyCode::Right.into()) {
            if player_x < max_x - 1 {
                player_x = player_x + 1;
                execute!(stdout, MoveTo(player_x * 2 - 2, player_y))?;
                println!("  ");
                execute!(stdout, MoveTo(player_x * 2, player_y))?;
                println!("🦀");
                if grid[player_x as usize][player_y as usize] == 2 {
                    // grid[player_x as usize][player_y as usize] = 0;
                    let mut i_x = rng.gen_range(0, max_x);
                    let mut i_y = rng.gen_range(0, max_y-1);

                    while i_x == player_x {
                        i_x = rng.gen_range(0, max_x);
                    }
                    while i_y == player_y {
                        i_y = rng.gen_range(0, max_y-1);
                    }
                    grid[i_x as usize][i_y as usize] = 2;
                    execute!(stdout, MoveTo(i_x * 2, i_y))?;
                    println!("🐛");
                }
            };
        }
        if event == Event::Key(KeyCode::Left.into()) {
            if player_x > 0 {
                player_x = player_x - 1;
                execute!(stdout, MoveTo(player_x * 2 + 2, player_y))?;
                println!("  ");
                execute!(stdout, MoveTo(player_x * 2, player_y))?;
                println!("🦀");
                if grid[player_x as usize][player_y as usize] == 2 {
                    // grid[player_x as usize][player_y as usize] = 0;
                    let mut i_x = rng.gen_range(0, max_x);
                    let mut i_y = rng.gen_range(0, max_y-1);

                    while i_x == player_x {
                        i_x = rng.gen_range(0, max_x);
                    }
                    while i_y == player_y {
                        i_y = rng.gen_range(0, max_y-1);
                    }
                    grid[i_x as usize][i_y as usize] = 2;
                    execute!(stdout, MoveTo(i_x * 2, i_y))?;
                    println!("🐛");
                }
            };
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
        SetForegroundColor(Color::Black),
        SetBackgroundColor(Color::Rgb {
            r: 250u8,
            g: 214u8,
            b: 76u8
        })
    )?;

    if let Err(e) = print_events() {
        println!("Error: {:?}\r", e);
    }

    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()
}
