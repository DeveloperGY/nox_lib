use std::time::Duration;
use nox_lib::graphics::prelude::*;
use nox_lib::io::Term;

fn main() {
    let width = 40;
    let height = 20;

    let mut gui = Gui::new(width, height);
    let mut term = Term::new().unwrap();

    term.save_state().unwrap();
    term.enable_raw_input().unwrap();
    gui.clear_terminal();
    gui.hide_cursor();

    

    let pos = (width / 2, height / 2);
    let mut x = 0;
    let mut y = 0;



    let mut running = true;
    while running {
        match term.getch(Duration::ZERO).unwrap() {
            None => (),
            Some(c) => {
                match c {
                    'w' => y -= 1,
                    's' => y += 1,
                    'a' => x -= 1,
                    'd' => x += 1,
                    'q' => running = false,
                    _ => ()
                }
            }
        };

        gui.clear(' ', Color::DEFAULT, Color::new(0, 0, 50));
        gui.line(pos.0, pos.1, pos.0 + x, pos.1 + y, '*', Color::WHITE, Color::DEFAULT);

        gui.display();
    }

    gui.show_cursor();
    term.disable_raw_input().unwrap();
    term.restore_state().unwrap();
}