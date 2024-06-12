use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

pub struct Renderer {
    pub screen_area: Rect,
    pub clear_color: Color,
}

impl Renderer {
    pub fn render(&self, canvas: &mut Canvas<Window>) {
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
}
