use sdl2::{event::Event, pixels::Color, rect::Rect};

mod view;
use view::board_view;

mod model;
use model::game::{make_blank_board, BoardPiece, GameState};

fn main() -> Result<(), String> {
    let screen_width: u32 = 800;
    let screen_height: u32 = 600;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Teeko", screen_width, screen_height)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let board_view = board_view::Renderer {
        screen_area: Rect::new(0, 0, screen_width, screen_height),
        clear_color: Color::RGB(64, 192, 255),
    };

    let mut game_state = GameState {
        board: make_blank_board(),
        current_player: BoardPiece::Red,
        pieces_dropped: [0, 0],
    };

    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();

    while running {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => running = false,
                Event::MouseButtonDown { x, y, .. } => {
                    let col: usize = (5 * x / board_view.screen_area.w).try_into().unwrap();
                    let row: usize = (5 * y / board_view.screen_area.h).try_into().unwrap();

                    game_state.handle_click(row, col);
                }
                _ => {}
            }
        }

        board_view.render(&mut canvas, &game_state.board);

        canvas.present();
    }

    Ok(())
}
