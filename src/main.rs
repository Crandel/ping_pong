extern crate ncurses;
extern crate ping_pong;

use ncurses::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use ping_pong::manager::Manager;

const BALL_COLOR: i16 = 1;
const PLAYER_COLOR: i16 = 2;
const NUMBER_COLOR: i16 = 3;

fn main() {
    /* Setup ncurses. */
    initscr();
    raw();

    /* Allow for extended keyboard (like F1). */
    keypad(stdscr(), true);
    noecho();

    /* Start colors. */
    start_color();
    init_pair(BALL_COLOR, COLOR_RED, COLOR_BLACK);
    init_pair(PLAYER_COLOR, COLOR_GREEN, COLOR_BLACK);
    init_pair(NUMBER_COLOR, COLOR_YELLOW, COLOR_BLACK);
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
                if manager.is_ball(chr) {
                    attron(COLOR_PAIR(BALL_COLOR));
                    mvprintw(i as i32, j as i32, chr);
                    attroff(COLOR_PAIR(BALL_COLOR));
                } else if manager.is_paddle(chr) {
                    attron(COLOR_PAIR(PLAYER_COLOR));
                    mvprintw(i as i32, j as i32, chr);
                    attroff(COLOR_PAIR(PLAYER_COLOR));
                } else if chr.parse::<i32>().is_ok() {
                    attron(COLOR_PAIR(NUMBER_COLOR));
                    mvprintw(i as i32, j as i32, chr);
                    attroff(COLOR_PAIR(NUMBER_COLOR));
                } else {
                    mvprintw(i as i32, j as i32, chr);
                }
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
