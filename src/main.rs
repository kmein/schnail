extern crate clap;
extern crate pancurses;
extern crate schnail;

use schnail::*;
use pancurses::*;
use std::iter::repeat;

fn main() {
    // this needs to come before ncurses is initialised. otherwise the usage text will look shite
    let matches = clap::App::new("schnail")
        .about("An exciting simulation of »Tempo, kleine Schnecke!«")
        .arg_from_usage("[dice] -d, --dice=[DICE] 'Number of dice (default 2)'")
        .arg_from_usage("[goal] 'Length of the race track (default 8)'")
        .get_matches();

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

    let mut board = Board::new(
        matches
            .value_of("goal")
            .and_then(|goal| goal.parse().ok())
            .unwrap_or(8),
    );
    let num_dice = matches.value_of("dice").and_then(|dice| dice.parse().ok()).unwrap_or(2);

    board.draw(&window);

    loop {
        window.clear();
        window.mvaddstr(8, 0, "dice ");

        let dice = repeat(roll).map(|d| d()).take(num_dice).collect::<Vec<_>>();
        for (idx, &colour) in dice.iter().enumerate() {
            window.with_colour_pair(colour as i32, || {
                window.mvaddch(8, 5 + 2*idx as i32, '#');
            });
            board.advance(colour);
        }

        board.draw(&window);

        let winners = board.winners();
        if winners.len() > 0 {
            window.mvaddstr(9, 0, "winner ");
            for (idx, &winner) in winners.iter().enumerate() {
                window.with_colour_pair(winner as i32, || {
                    window.mvaddch(9, 7 + 2*idx as i32, '#');
                });
            }
            break;
        }
        window.getch();
    }

    window.refresh();
    window.getch();
    endwin();
}
