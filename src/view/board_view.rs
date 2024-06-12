use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::model::game::BoardPiece;

pub struct Renderer {
    pub screen_area: Rect,
    pub clear_color: Color,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            screen_area: Rect::new(0, 0, width, height),
            clear_color: Color::RGB(64, 192, 255),
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, board: &[[BoardPiece; 5]; 5]) {
        // Background
        canvas.set_draw_color(self.clear_color);
        canvas.fill_rect(self.screen_area).ok().unwrap_or_default();

        // Lines
        self.draw_lines(canvas);

        // Pieces
        self.draw_pieces(canvas, board);
    }

    pub fn draw_lines(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.clear_color);
        canvas.fill_rect(self.screen_area).ok().unwrap_or_default();

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        let cell_width: i32 = self.screen_area.w / 5;
        let cell_height: i32 = self.screen_area.h / 5;

        for i in 0..5 {
            let h_x = cell_width / 2;
            let h_y = cell_height / 2 + i * cell_height;

            let v_x = cell_width / 2 + i * cell_width;
            let v_y = cell_height / 2;

            // Horizontal
            canvas
                .draw_line(
                    Point::new(h_x, h_y),
                    Point::new(self.screen_area.w - h_x, h_y),
                )
                .ok()
                .unwrap_or_default();

            // Vertical
            canvas
                .draw_line(
                    Point::new(v_x, v_y),
                    Point::new(v_x, self.screen_area.h - v_y),
                )
                .ok()
                .unwrap_or_default();

            // Diagonal up-right a
            canvas
                .draw_line(Point::new(h_x, h_y), Point::new(v_x, v_y))
                .ok()
                .unwrap_or_default();

            // Diagonal up-right b
            canvas
                .draw_line(
                    Point::new(v_x, self.screen_area.h - v_y),
                    Point::new(self.screen_area.w - h_x, h_y),
                )
                .ok()
                .unwrap_or_default();

            // Diagonal up-down a
            canvas
                .draw_line(
                    Point::new(h_x, h_y),
                    Point::new(self.screen_area.w - v_x, self.screen_area.h - v_y),
                )
                .ok()
                .unwrap_or_default();

            // Diagonal up-down b
            canvas
                .draw_line(
                    Point::new(v_x, v_y),
                    Point::new(self.screen_area.w - h_x, self.screen_area.h - h_y),
                )
                .ok()
                .unwrap_or_default();
        }
    }

    fn draw_pieces(&self, canvas: &mut Canvas<Window>, board: &[[BoardPiece; 5]; 5]) {
        let width = self.screen_area.w / 5;
        let height = self.screen_area.h / 5;

        for i in 0i32..5 {
            let row: usize = i.try_into().unwrap();

            for j in 0i32..5 {
                let col: usize = j.try_into().unwrap();

                if board[row][col] == BoardPiece::None {
                    continue;
                }

                let mut color = Color::RGB(0, 0, 0);
                if board[row][col] == BoardPiece::Red {
                    color = Color::RGB(255, 0, 0);
                }

                let rect = Rect::new(
                    width / 4 + width * j,
                    height / 4 + height * i,
                    (width / 2).try_into().unwrap(),
                    (height / 2).try_into().unwrap(),
                );

                canvas.set_draw_color(color);
                canvas.fill_rect(rect).ok().unwrap_or_default();
            }
        }
    }
}
