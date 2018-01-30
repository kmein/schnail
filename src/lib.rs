extern crate pancurses;
extern crate rand;

use pancurses::{Window, COLOR_PAIR};
use rand::Rng;
use std::collections::HashMap;
use std::iter::repeat;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy)]
pub enum Colour {
    Red,
    Yellow,
    Green,
    Pink,
    Blue,
    Orange,
}

fn from_colour_code(code: i32) -> Option<Colour> {
    match code {
        0 => Some(Colour::Red),
        1 => Some(Colour::Yellow),
        2 => Some(Colour::Green),
        3 => Some(Colour::Pink),
        4 => Some(Colour::Blue),
        5 => Some(Colour::Orange),
        _ => None,
    }
}

pub fn roll() -> Colour {
    let n = rand::thread_rng().gen_range(0, 6);
    from_colour_code(n).unwrap()
}

pub trait WindowExt {
    fn with_colour_pair<F: Fn()>(&self, colour: i32, action: F);
}

impl WindowExt for Window {
    fn with_colour_pair<F: Fn()>(&self, colour: i32, action: F) {
        let pair = COLOR_PAIR(colour as u64);
        self.attron(pair);
        action();
        self.attroff(pair);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Board {
    scale: i32,
    goal: i32,
    snails: HashMap<Colour, i32>,
}

impl Board {
    pub fn new(goal: i32) -> Self {
        use Colour::*;

        let scale = (goal as f32).log10() as i32 + 2;

        let mut snails = HashMap::new();
        for colour in vec![Red, Yellow, Green, Pink, Blue, Orange] {
            snails.insert(colour, 0);
        }

        Board {
            snails,
            scale,
            goal,
        }
    }

    pub fn advance(&mut self, colour: Colour) {
        self.snails.get_mut(&colour).map(|s| *s += 1);
    }

    pub fn winner(&self) -> Option<Colour> {
        self.snails.iter().find(|s| *s.1 == self.goal).map(|s| *s.0)
    }

    pub fn draw(&self, window: &Window) {
        for y in 0..6 {
            window.attron(COLOR_PAIR(0));
            window.mvaddch(y, self.scale, '|');
            window.mvaddch(y, self.goal * self.scale, '|');

            let colour = from_colour_code(y).unwrap();
            window.with_colour_pair(y, || {
                window.mvaddstr(
                    y,
                    self.snails[&colour] * self.scale,
                    &repeat('@').take(self.scale as usize).collect::<String>(),
                );
            });
        }

        for x in 0..self.goal + 1 {
            window.mvaddstr(6, x * self.scale, &format!("{}", x));
        }
    }
}
