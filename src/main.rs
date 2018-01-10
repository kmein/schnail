extern crate schnail;
extern crate pancurses;

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

        let (dice1, dice2) = (roll(), roll());
        {
            let code1 = to_colour_code(&dice1) as u64;
            let code2 = to_colour_code(&dice2) as u64;

            window.mvaddstr(8, 0, "dice ");
            window.attron(COLOR_PAIR(1 + code1));

            window.mvaddch(8, 5, '#');
            window.attroff(COLOR_PAIR(1 + code1));

            window.mvaddch(8, 6, ' ');
            window.attron(COLOR_PAIR(1 + code2));

            window.mvaddch(8, 7, '#');
            window.attroff(COLOR_PAIR(1 + code2));
        }

        board.advance(dice1);
        board.advance(dice2);
        board.draw(&window);
        if let Some(winner) = board.winner() {
            // window.mvaddstr(to_colour_code(&winner) as i32, GOAL as i32*2+8, "winner");
            let winner_code = to_colour_code(&winner) as u64;
            window.mvaddstr(9, 0, "winner ");
            window.attron(COLOR_PAIR(1 + winner_code));
            window.mvaddch(9, 7, '#');
            window.attroff(COLOR_PAIR(1 + winner_code));
            break;
        }
        window.getch();
    }

    window.refresh();
    window.getch();
    endwin();
}
