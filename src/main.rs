extern crate ncurses;
extern crate ping_pong;

use ncurses::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use ping_pong::manager::Manager;

fn main() {
    /* Setup ncurses. */
    initscr();
    raw();

    /* Allow for extended keyboard (like F1). */
    keypad(stdscr(), true);
    noecho();

    /* Invisible cursor. */
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    refresh();
    /* Get the screen bounds. */
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    let mut manager = Manager::new(max_y, max_x);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || loop {
        let ch = getch();
        tx.send(ch).unwrap();
    });

    let mut received = rx.try_iter();
    while !manager.get_quit() {
        let line_box = manager.draw();
        for (i, line) in line_box.iter().enumerate() {
            for (j, chr) in line.iter().enumerate() {
                mvprintw(i as i32, j as i32, chr);
            }
        }
        let ch = received.next();
        manager.controls(ch);

        manager.ball_movements();
        refresh();
        thread::sleep(Duration::from_millis(100));
    }
    curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
    echo();
    refresh();
}
