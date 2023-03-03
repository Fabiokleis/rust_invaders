use std::{error::Error, time::{Duration, Instant}, sync::mpsc, thread};
use crossterm::{terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, cursor::{Hide, Show}, event::{self, KeyCode, Event}};
use invaders::{frame::{self, Drawable}, render, player::Player, invaders::Invaders};

fn main() -> Result<(), Box<dyn Error>>{

    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;
    
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move ||{
        let mut last_frame = frame::new_frame();
        let mut stdout = std::io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });



    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    'gl: loop {
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = frame::new_frame();
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gl;
                    },
                    KeyCode::Left => {
                        player.move_left();
                    },
                    KeyCode::Right => {
                        player.move_right();
                    },
                    KeyCode::Up => {
                        player.shoot();
                    }
                    _ => {}
                }
            }
        }
        player.update(delta);
        invaders.update(delta);
        player.detect_hits(&mut invaders);
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        if invaders.all_killed() {
            break 'gl;
        }
        if invaders.reached_bottom() {
            break 'gl;
        }
    }

    drop(render_tx);
    render_handle.join().unwrap();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
