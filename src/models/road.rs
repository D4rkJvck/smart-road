use sdl2::rect::Point;

use crate::{GAP, HEIGHT, MID_HEIGHT, MID_WIDTH, WIDTH};

use super::Intersection;

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(x1: u32, y1: u32, x2: u32, y2: u32) -> Self {
        Self {
            start: Point::new(x1 as i32, y1 as i32),
            end: Point::new(x2 as i32, y2 as i32),
        }
    }
}

//________________________________________________________________
//

pub struct Road {
    pub lines: Vec<Line>,
    pub intersection: Intersection,
}

impl Road {
    pub fn new() -> Self {
        let lines = vec![
            // Vertical lines
            Line::new(MID_WIDTH - GAP * 3, 0, MID_WIDTH - GAP * 3, WIDTH),
            Line::new(MID_WIDTH - GAP * 2, 0, MID_WIDTH - GAP * 2, WIDTH),
            Line::new(MID_WIDTH - GAP, 0, MID_WIDTH - GAP, WIDTH),
            Line::new(MID_WIDTH, 0, MID_WIDTH, WIDTH),
            Line::new(MID_WIDTH + GAP, 0, MID_WIDTH + GAP, WIDTH),
            Line::new(MID_WIDTH + GAP * 2, 0, MID_WIDTH + GAP * 2, WIDTH),
            Line::new(MID_WIDTH + GAP * 3, 0, MID_WIDTH + GAP * 3, WIDTH),
            // Horizontal Lines
            Line::new(0, MID_HEIGHT - GAP * 3, HEIGHT, MID_HEIGHT - GAP * 3),
            Line::new(0, MID_HEIGHT - GAP * 2, HEIGHT, MID_HEIGHT - GAP * 2),
            Line::new(0, MID_HEIGHT - GAP, HEIGHT, MID_HEIGHT - GAP),
            Line::new(0, MID_HEIGHT, HEIGHT, MID_HEIGHT),
            Line::new(0, MID_HEIGHT + GAP, HEIGHT, MID_HEIGHT + GAP),
            Line::new(0, MID_HEIGHT + GAP * 2, HEIGHT, MID_HEIGHT + GAP * 2),
            Line::new(0, MID_HEIGHT + GAP * 3, HEIGHT, MID_HEIGHT + GAP * 3),
        ];

        let intersection = Intersection::new();

        Self {
            lines,
            intersection,
        }
    }
}
