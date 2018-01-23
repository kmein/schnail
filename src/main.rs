extern crate pancurses;
extern crate schnail;

use schnail::*;
use pancurses::*;

fn main() {
    let window = initscr();
    start_color();
    curs_set(0);

    // foreground colours
    init_pair(1, COLOR_RED, COLOR_BLACK);
    init_pair(2, COLOR_YELLOW, COLOR_BLACK);
    init_pair(3, COLOR_GREEN, COLOR_BLACK);
    init_pair(4, COLOR_MAGENTA, COLOR_BLACK);
    init_pair(5, COLOR_BLUE, COLOR_BLACK);
    init_pair(6, COLOR_WHITE, COLOR_BLACK);

    let mut board = Board::new();
    board.draw(&window);

    loop {
        window.clear();

        let dice = (roll(), roll());

        window.mvaddstr(8, 0, "dice ");
        window.with_colour_pair(to_colour_code(&dice.0), || {
            window.mvaddch(8, 5, '#');
        });
        window.mvaddch(8, 6, ' ');
        window.with_colour_pair(to_colour_code(&dice.1), || {
            window.mvaddch(8, 7, '#');
        });

        board.advance(dice.0);
        board.advance(dice.1);
        board.draw(&window);
        if let Some(winner) = board.winner() {
            window.mvaddstr(9, 0, "winner ");
            window.with_colour_pair(to_colour_code(&winner), || {
                window.mvaddch(9, 7, '#');
            });
            break;
        }
        window.getch();
    }

    window.refresh();
    window.getch();
    endwin();
}
