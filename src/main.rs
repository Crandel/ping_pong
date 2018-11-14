extern crate console;
extern crate ncurses;
extern crate ping_pong;

use console::Term;
use ncurses::*;

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

    /* Status/help info. */
    printw("Use the arrow keys to move");
    mvprintw(LINES() - 1, 0, "Press F1 to exit");
    refresh();

    /* Get the screen bounds. */
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    let mut manager = Manager::new(max_x as u16, max_y as u16);

    let mut ch = getch();

    while !manager.get_quit() {
        // let line_box = self.draw();
        // for line in line_box {
        //     term.write_line(&line).unwrap();
        // }
        manager.controls(ch);
        manager.ball_movements();
        // thread::sleep(Duration::from_millis(100));
        ch = getch();
    }
}
