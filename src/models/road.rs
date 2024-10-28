use sdl2::rect::Point;

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self {
            start: Point::new(x1, y1),
            end: Point::new(x2, y2),
        }
    }
}

//________________________________________________________________
//

pub struct Road {
    pub lines: Vec<Line>,
}

impl Road {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            lines: vec![
                // Vertical lines
                Line::new(300, 0, 300, 900),
                Line::new(350, 0, 350, 900),
                Line::new(400, 0, 400, 900),
                Line::new(450, 0, 450, 900),
                Line::new(500, 0, 500, 900),
                Line::new(550, 0, 550, 900),
                Line::new(600, 0, 600, 900),

                // Horizontal Lines
                Line::new(0, 300, 900, 300),
                Line::new(0, 350, 900, 350),
                Line::new(0, 400, 900, 400),
                Line::new(0, 450, 900, 450),
                Line::new(0, 500, 900, 500),
                Line::new(0, 550, 900, 550),
                Line::new(0, 600, 900, 600),
            ],
        })
    }
}
