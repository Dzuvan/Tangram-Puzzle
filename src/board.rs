// Modul za rad s tablom za slagalicu.

extern crate sdl2;

use std::process;

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use ::sdl2::messagebox::*;

use shapes::*;
use constants::*;

pub struct Board {
    x: i32,
    y: i32,
}

impl Board {
    pub fn new() -> Board {
        Board{
            x: MATRIX_SIZE,
            y: MATRIX_SIZE,
        }
    }

    pub fn draw_grid(&self, canvas:&mut Canvas<Window>, color: Color) {
        canvas.set_draw_color(color);

        for cell_x in 0..self.x{
            for cell_y in 0..self.y{
                let col_1 = Rect::new((cell_x+1)* DIMENSION,(cell_y as i32 + 1) * DIMENSION , DIMENSION as u32, DIMENSION as u32);
                let col_2 = Rect::new((cell_x+1) * DIMENSION,(cell_y as i32 + 1) * DIMENSION , DIMENSION as u32, DIMENSION as u32);
                let col_3 = Rect::new((cell_x+1)* DIMENSION,(cell_y as i32 + 1) * DIMENSION , DIMENSION as u32, DIMENSION as u32);
                let col_4 = Rect::new((cell_x+1) * DIMENSION,(cell_y as i32 + 1) * DIMENSION , DIMENSION as u32, DIMENSION as u32);
                let col_5 = Rect::new((cell_x+1) * DIMENSION,(cell_y as i32 + 1) * DIMENSION , DIMENSION as u32, DIMENSION as u32);

                canvas.draw_rect(col_1).unwrap();
                canvas.draw_rect(col_2).unwrap();
                canvas.draw_rect(col_3).unwrap();
                canvas.draw_rect(col_4).unwrap();
                canvas.draw_rect(col_5).unwrap();

            }
        }
    }
    pub fn end_screen(&self, s: &mut SShape, u:&mut UShape, i:&mut IShape,g:&mut GShape, f:&mut FShape, l:&mut LShape, r:&mut RShape) {
        let buttons : Vec<_> = vec![
            ButtonData {
                flags:MESSAGEBOX_BUTTON_NOTHING,
                button_id:1,
                text:"Play Again"
            },
        ];

        let res = show_message_box(MESSAGEBOX_INFORMATION,
                                   buttons.as_slice(),
                                   "Victory",
                                   "Congratulations for finishing such a perilous journey. Give yourself a tap on the shoulder.",
                                   None,
                                   None);

        for b in &buttons {
            if b.button_id == 1 {
                s.shuffle();
                u.shuffle();
                i.shuffle();
                g.shuffle();
                f.shuffle();
                l.shuffle();
                r.shuffle();
           }
           }
        }
}
