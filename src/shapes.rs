// Modul za kreiranje i manipulaciju delovima slagalice.
// Rust nema nasleđivanje i polimorfizam kao OOP jezici,
// zbog čega je fajl veći nego što bi trebao da bude.

extern crate rand;
extern crate sdl2;

use std::process;

use rand::Rng;

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::event::Event;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

use constants::*;

// Interfejs koji implementiraju strukture koje predstavljaju delove slagalice.
pub trait Piece {
    fn new(x:i32, y: i32)->Self where Self:Sized;
    fn draw(&self, canvas: &mut Canvas<Window>, color: Color) where Self:Sized;
    fn intersects(&self, object_x: i32, object_y: i32,dimensions: i32, mouse_x: i32, mouse_y: i32)->bool where Self:Sized{
        if mouse_x < object_x || mouse_y < object_y {
            return false;
        }
        if mouse_x >  object_x + dimensions  || mouse_y >  object_y + dimensions{
            return false;
        }
        true
    }
    fn stop_drag(&mut self);
    fn handle_events(&mut self, &Event);
    fn reposition(&mut self, i32, i32);
    fn shuffle(&mut self);
    fn check_win(&self, &[i32;2])->bool;
}
pub struct SShape {
    pub is_dragging: bool,

    pub offset_x: i32,
    pub offset_y: i32,

    pub bottom_left_x: i32,
    pub bottom_left_y: i32,

    pub bottom_right_x: i32,
    pub bottom_right_y: i32,

    pub top_left_x: i32,
    pub top_left_y: i32,

    pub top_right_x: i32,
    pub top_right_y: i32,
}

impl Piece for  SShape {
    fn check_win(&self, coords: &[i32;2]) -> bool{
       if self.bottom_left_x == coords[0] && self.bottom_left_y == coords[1] {
           return true;
       }
       false
    }
    fn shuffle(&mut self) {
         self.bottom_left_x = rand::thread_rng().gen_range(700, WIDTH as i32 - (2*DIMENSION));
         self.bottom_left_y = rand::thread_rng().gen_range(DIMENSION, HEIGHT as i32 - (2*DIMENSION));
         let x = self.bottom_left_x;
         let y = self.bottom_left_y;
         self.reposition(x,y);
    }

    fn new(x:i32, y:i32) -> SShape {
        let bottom_left_x = x;
        let bottom_left_y = y;
        let bottom_right_x = bottom_left_x + DIMENSION;
        let bottom_right_y = bottom_left_y;

        let top_left_x = bottom_left_x;
        let top_left_y = bottom_left_y - DIMENSION;

        let top_right_x = bottom_left_x + DIMENSION;
        let top_right_y= bottom_left_y - DIMENSION;

        SShape {
            is_dragging: false,
            offset_x: 0,
            offset_y: 0,
            bottom_left_x: bottom_left_x,
            bottom_left_y: bottom_left_y,
            bottom_right_x: bottom_right_x,
            bottom_right_y: bottom_right_y,
            top_left_x: top_left_x,
            top_left_y: top_left_y,
            top_right_x: top_right_x,
            top_right_y: top_right_y,
        }
    }

    fn stop_drag(&mut self) {
        self.is_dragging = false;
        self.offset_x = 0;
        self.offset_y = 0;
    }

    fn draw(&self, canvas: &mut Canvas<Window>, color: Color) {
        let left_bottom = Rect::new(self.bottom_left_x,self.bottom_left_y, DIMENSION as u32 , DIMENSION as u32 );
        let right_bottom = Rect::new(self.bottom_right_x, self.bottom_right_y, DIMENSION as u32, DIMENSION as u32);
        let top_left = Rect::new(self.top_left_x ,self.top_left_y, DIMENSION as u32, DIMENSION as u32);
        let top_right = Rect::new(self.top_right_x,self.top_right_y, DIMENSION as u32, DIMENSION as u32);
        let rects = [left_bottom, right_bottom,top_left, top_right];
        canvas.set_draw_color(color);
        canvas.draw_rects(&rects).ok();
        canvas.fill_rects(&rects).ok();
    }

    fn reposition(&mut self, x:i32, y: i32) {
        self.bottom_right_x = x + DIMENSION;
        self.bottom_right_y = y;
        self.top_left_x = x;
        self.top_left_y = y - DIMENSION;
        self.top_right_x = x + DIMENSION;
        self.top_right_y = y - DIMENSION;
    }

    fn handle_events(&mut self, event: &Event) {
        match *event {
            Event::Quit {..}| Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                process::exit (0x0f00);
            },
            Event::MouseButtonDown{x,y, mouse_btn, ..} => {
                if mouse_btn == MouseButton::Left && self.intersects(self.bottom_left_x, self.bottom_left_y, DIMENSION,x, y)
                                                || self.intersects(self.bottom_right_x, self.bottom_right_y, DIMENSION, x, y)
                                                ||self.intersects(self.top_left_x, self.top_left_y, DIMENSION, x, y)
                                                ||self.intersects(self.top_right_x, self.top_right_y, DIMENSION, x, y) {
                            self.offset_x = x - self.bottom_left_x;
                            self.offset_y = y - self.bottom_left_y;
                            self.is_dragging = true;
                        }
            },
            Event::MouseButtonUp{mouse_btn,..} => {
                if mouse_btn == MouseButton::Left && self.is_dragging {
                    self.stop_drag();
                    if  self.bottom_left_x + DIMENSION > DIMENSION && self.bottom_right_x < DIMENSION * (MATRIX_SIZE+1) &&
                        self.bottom_left_y > DIMENSION && self.top_right_y < (DIMENSION *  (MATRIX_SIZE+1)) {
                            self.bottom_left_x = (self.bottom_left_x / DIMENSION) * DIMENSION;
                            self.bottom_left_y = (self.bottom_left_y / DIMENSION) * DIMENSION;
                            let x = self.bottom_left_x;
                            let y = self.bottom_left_y;
                            self.reposition(x, y);
                            self.check_win(&[x, y]);
                        }
                }
            },
            Event::MouseMotion{x, y,..} => {
                if self.is_dragging {
                    self.bottom_left_x = x - self.offset_x;
                    self.bottom_left_y = y - self.offset_y;
                    let x = self.bottom_left_x;
                    let y = self.bottom_left_y;
                    self.reposition(x,y);
                }
            },
            Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                self.shuffle();
            }
            _ => {}
        }
    }
}

pub struct UShape {
    pub is_dragging: bool,
    pub offset_x: i32,
    pub offset_y: i32,

    pub left_x: i32,
    pub left_y: i32,

    pub right_x: i32,
    pub right_y: i32,
}
impl  Piece for UShape {
    fn check_win(&self, coords: &[i32;2]) -> bool{
       if self.left_x == coords[0] && self.left_y == coords[1] {
           return true
       }
       false
    }
    fn shuffle(&mut self) {
         self.left_x = rand::thread_rng().gen_range(700, WIDTH as i32 - (2*DIMENSION));
         self.left_y = rand::thread_rng().gen_range(DIMENSION, HEIGHT as i32 - (2*DIMENSION));
         let x = self.left_x;
         let y = self.left_y;
         self.reposition(x,y);
    }
    fn new(x: i32, y: i32) -> UShape {
        let left_x = x;
        let left_y = y;
        let right_x = left_x + DIMENSION;
        let right_y = left_y;

        UShape {
            is_dragging: false,
            offset_x: 0,
            offset_y: 0,
            left_x: left_x,
            left_y: left_y,
            right_x: right_x,
            right_y: right_y,
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>, color: Color) {
        let left = Rect::new(self.left_x,self.left_y, DIMENSION as u32 , DIMENSION as u32 );
        let right= Rect::new(self.right_x, self.right_y, DIMENSION as u32, DIMENSION as u32);
        let rects =[left, right];
        canvas.set_draw_color(color);
        canvas.draw_rects(&rects).ok();
        canvas.fill_rects(&rects).ok();
    }

    fn stop_drag(&mut self) {
        self.is_dragging = false;
        self.offset_x = 0;
        self.offset_y = 0;
    }

    fn reposition(&mut self, x: i32, y: i32) {
            self.right_x = x + DIMENSION;
            self.right_y = y;
    }

    fn handle_events(&mut self, event: &Event) {
        match *event {
            Event::Quit {..}| Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                process::exit (0x0f00);
            },
            Event::MouseButtonDown{x,y, mouse_btn, ..} => {
                if mouse_btn == MouseButton::Left && self.intersects(self.left_x,self.left_y, DIMENSION, x, y)
                                                    || self.intersects(self.right_x,self.right_y, DIMENSION, x, y) {
                        self.offset_x = x - self.left_x;
                        self.offset_y = y - self.left_y;
                        self.is_dragging = true;
                }
            },
            Event::MouseButtonUp{mouse_btn,..} => {
                if mouse_btn == MouseButton::Left && self.is_dragging {
                    self.stop_drag();
                    if  self.left_x + DIMENSION > DIMENSION && self.right_x < DIMENSION * (MATRIX_SIZE+1) &&
                                                    self.left_y > DIMENSION && self.right_y < DIMENSION *( MATRIX_SIZE+1) {
                            self.left_x = (self.left_x / DIMENSION) * DIMENSION;
                            self.left_y = (self.left_y / DIMENSION) * DIMENSION;
                            let x = self.left_x;
                            let y = self.left_y;
                            self.reposition(x, y);
                            self.check_win(&[x, y]);
                    }
                }
            },
            Event::MouseMotion{x, y,..} => {
                if self.is_dragging {
                    self.left_x = x - self.offset_x;
                    self.left_y = y - self.offset_y;
                    let x = self.left_x;
                    let y = self.left_y;
                    self.reposition(x, y);
                }
            },
            Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                self.shuffle();
            }
            _ => {}
        }
    }
}
pub struct IShape {
    pub is_dragging: bool,
    pub offset_x: i32,
    pub offset_y: i32,

    pub left_x: i32,
    pub left_y: i32,

    pub top_x: i32,
    pub top_y: i32,
}

impl  Piece for IShape {
    fn check_win(&self, coords: &[i32;2]) -> bool {
       if self.left_x == coords[0] && self.left_y == coords[1] {
           return true;
       }
       false
    }
    fn shuffle(&mut self) {
         self.left_x = rand::thread_rng().gen_range(700, WIDTH as i32 - (2*DIMENSION));
         self.left_y = rand::thread_rng().gen_range(DIMENSION, HEIGHT as i32 - (2*DIMENSION));
         let x = self.left_x;
         let y = self.left_y;
         self.reposition(x,y);
    }
    fn new(x:i32, y: i32) -> IShape {
        let left_x = x;
        let left_y = y;
        let top_x = left_x ;
        let top_y = left_y - DIMENSION;

        IShape {
            is_dragging: false,
            offset_x: 0,
            offset_y: 0,
            left_x: left_x,
            left_y: left_y,
            top_x: top_x,
            top_y: top_y,
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>, color: Color) {
        let left = Rect::new(self.left_x, self.left_y, DIMENSION as u32 , DIMENSION as u32 );
        let right = Rect::new(self.top_x, self.top_y, DIMENSION as u32, DIMENSION as u32);
        let rects = [left, right];
        canvas.set_draw_color(color);
        canvas.draw_rects(&rects).ok();
        canvas.fill_rects(&rects).ok();
    }

    fn stop_drag(&mut self) {
        self.is_dragging = false;
        self.offset_x = 0;
        self.offset_y = 0;
    }

    fn reposition(&mut self, x: i32, y: i32) {
        self.top_x = x;
        self.top_y = y + DIMENSION;
    }

    fn handle_events(&mut self, event: &Event) {
        match *event {
            Event::Quit {..}| Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                process::exit (0x0f00);
            },
            Event::MouseButtonDown{x,y, mouse_btn, ..} => {
                if mouse_btn == MouseButton::Left && self.intersects(self.left_x,self.left_y, DIMENSION,x, y)
                                                    || self.intersects(self.top_x, self.top_y, DIMENSION,x, y) {
                        self.offset_x = x - self.left_x;
                        self.offset_y = y - self.left_y;
                        self.is_dragging = true;
                    }

            },
            Event::MouseButtonUp{mouse_btn,..} => {
                if mouse_btn == MouseButton::Left && self.is_dragging {
                    self.stop_drag();
                    if  self.left_x > DIMENSION && self.left_x + DIMENSION < DIMENSION * (MATRIX_SIZE+2) &&
                        self.left_y > DIMENSION && self.top_y + DIMENSION< DIMENSION * (MATRIX_SIZE +2 ) {
                            self.left_x = (self.left_x / DIMENSION) * DIMENSION;
                            self.left_y = (self.left_y / DIMENSION) * DIMENSION;
                            let x = self.left_x;
                            let y = self.left_y;
                            self.reposition(x, y);
                            self.check_win(&[x, y]);
                    }
                }
            },
            Event::MouseMotion{x, y,..} => {
                if self.is_dragging {
                    self.left_x = x - self.offset_x;
                    self.left_y = y - self.offset_y;
                    let x  = self.left_x;
                    let y  = self.left_y;
                    self.reposition(x, y);
                }
            },
            Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                self.shuffle();
            }
            _ => {}
        }
    }
}

pub struct GShape {
    pub is_dragging: bool,
    pub offset_x: i32,
    pub offset_y: i32,

    pub bottom_left_x: i32,
    pub bottom_left_y: i32,

    pub bottom_right_x: i32,
    pub bottom_right_y: i32,

    pub top_left_x: i32,
    pub top_left_y: i32,

    pub top_right_x: i32,
    pub top_right_y: i32,
}

impl Piece for GShape {
    fn check_win(&self, coords: &[i32;2]) -> bool{
       if self.bottom_left_x == coords[0] && self.bottom_left_y == coords[1] {
           return true;
       }
       false
    }
    fn shuffle(&mut self) {
         self.bottom_left_x = rand::thread_rng().gen_range(700, WIDTH as i32 - (2*DIMENSION));
         self.bottom_left_y = rand::thread_rng().gen_range(DIMENSION, HEIGHT as i32 - (2*DIMENSION));
         let x = self.bottom_left_x;
         let y = self.bottom_left_y;
         self.reposition(x,y);
    }
    fn new(x: i32, y: i32) -> GShape {
        let bottom_left_x = x;
        let bottom_left_y = y;
        let bottom_right_x = bottom_left_x;
        let bottom_right_y = bottom_left_y - DIMENSION;
        let top_left_x = bottom_left_x;
        let top_left_y = bottom_left_y - (2*DIMENSION);
        let top_right_x = bottom_left_x - DIMENSION;
        let top_right_y = bottom_left_y - (2*DIMENSION);

        GShape {
            is_dragging: false,
            offset_x: 0,
            offset_y: 0,
            bottom_left_x: bottom_left_x,
            bottom_left_y: bottom_left_y,
            bottom_right_x: bottom_right_x,
            bottom_right_y: bottom_right_y,
            top_left_x: top_left_x,
            top_left_y: top_left_y,
            top_right_x: top_right_x,
            top_right_y: top_right_y,
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>, color: Color) {
        let l_b = Rect::new(self.bottom_left_x,self.bottom_left_y, DIMENSION as u32 , DIMENSION as u32 );
        let l_t = Rect::new(self.bottom_right_x, self.bottom_right_y, DIMENSION as u32, DIMENSION as u32);
        let r_b = Rect::new(self.top_left_x ,self.top_left_y, DIMENSION as u32, DIMENSION as u32);
        let r_t = Rect::new(self.top_right_x ,self.top_right_y , DIMENSION as u32, DIMENSION as u32);
        let rects = [l_b, l_t,r_b, r_t];
        canvas.set_draw_color(color);
        canvas.draw_rects(&rects).ok();
        canvas.fill_rects(&rects).ok();
    }

    fn stop_drag(&mut self) {
        self.is_dragging = false;
        self.offset_x = 0;
        self.offset_y = 0;
    }

    fn reposition(&mut self, x: i32, y: i32) {
            self.bottom_right_x = x;
            self.bottom_right_y = y -DIMENSION;
            self.top_left_x = x;
            self.top_left_y = y - (2*DIMENSION);
            self.top_right_x = x - DIMENSION;
            self.top_right_y = y - (2*DIMENSION);
    }

    fn handle_events(&mut self, event: &Event) {
        match *event {
            Event::Quit {..}| Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                process::exit (0x0f00);
            },
            Event::MouseButtonDown{x,y, mouse_btn, ..} => {
                if mouse_btn == MouseButton::Left && self.intersects(self.bottom_left_x,self.bottom_left_y, DIMENSION,x, y)
                                                    || self.intersects(self.bottom_right_x, self.bottom_right_y, DIMENSION, x, y)
                                                    || self.intersects(self.top_left_x, self.top_left_y, DIMENSION, x, y)
                                                    || self.intersects(self.top_right_x, self.top_right_y, DIMENSION, x, y) {
                            self.offset_x = x - self.bottom_left_x;
                            self.offset_y = y - self.bottom_left_y;
                            self.is_dragging = true;
                        }

            },
            Event::MouseButtonUp{mouse_btn,..} => {
                if mouse_btn == MouseButton::Left && self.is_dragging {
                    self.stop_drag();
                    if  self.bottom_left_x + DIMENSION > DIMENSION  && self.bottom_left_y-300 < DIMENSION * (MATRIX_SIZE+1) &&
                        self.top_right_x + DIMENSION > DIMENSION && self.top_right_y < DIMENSION * (MATRIX_SIZE+1) {
                            self.bottom_left_x = (self.bottom_left_x / DIMENSION) * DIMENSION;
                            self.bottom_left_y = (self.bottom_left_y / DIMENSION) * DIMENSION;
                            let x = self.bottom_left_x;
                            let y = self.bottom_left_y;
                            self.reposition(x, y);
                            self.check_win(&[x, y]);
                    }
                }
            },
            Event::MouseMotion{x, y,..} => {
                if self.is_dragging {
                    self.bottom_left_x = x - self.offset_x;
                    self.bottom_left_y = y - self.offset_y;
                    let x = self.bottom_left_x;
                    let y = self.bottom_left_y;
                    self.reposition(x, y);
                }
            },
            Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                self.shuffle();
            }
            _ => {}
        }
    }
}

pub struct FShape {
    is_dragging: bool,
    offset_x: i32,
    offset_y: i32,

    pub bottom_left_x: i32,
    pub bottom_left_y: i32,

    bottom_right_x: i32,
    bottom_right_y: i32,

    top_left_x: i32,
    top_left_y: i32,

    top_right_x: i32,
    top_right_y: i32,

    top_top_left_x: i32,
    top_top_left_y: i32,

}
impl Piece for FShape {
    fn check_win(&self, coords: &[i32;2]) -> bool{
       if self.bottom_left_x == coords[0] && self.bottom_left_y == coords[1] {
           return true
       }
       false
    }
    fn shuffle(&mut self) {
         self.bottom_left_x = rand::thread_rng().gen_range(700, WIDTH as i32 - (2*DIMENSION));
         self.bottom_left_y = rand::thread_rng().gen_range(DIMENSION, HEIGHT as i32 - (2*DIMENSION));
         let x = self.bottom_left_x;
         let y = self.bottom_left_y;
         self.reposition(x,y);
    }
    fn new(x: i32, y: i32) -> FShape {

        let bottom_left_x = x;
        let bottom_left_y = y;
        let bottom_right_x = bottom_left_x + DIMENSION;
        let bottom_right_y = bottom_left_y;

        let top_left_x = bottom_left_x;
        let top_left_y = bottom_left_y - DIMENSION;

        let top_right_x = bottom_left_x + DIMENSION;
        let top_right_y = bottom_left_y - DIMENSION;

        let top_top_left_x = bottom_left_x;
        let top_top_left_y = bottom_left_y - (2*DIMENSION);

        FShape {
            is_dragging: false,
            offset_x: 0,
            offset_y: 0,
            bottom_left_x: bottom_left_x,
            bottom_left_y: bottom_left_y,
            bottom_right_x: bottom_right_x,
            bottom_right_y: bottom_right_y,
            top_left_x: top_left_x,
            top_left_y: top_left_y,
            top_right_x: top_right_x,
            top_right_y: top_right_y,
            top_top_left_x: top_top_left_x,
            top_top_left_y: top_top_left_y,
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>, color: Color) {
        let l_b = Rect::new(self.bottom_left_x,self.bottom_left_y, DIMENSION as u32 , DIMENSION as u32 );
        let l_t = Rect::new(self.bottom_right_x, self.bottom_right_y, DIMENSION as u32, DIMENSION as u32);
        let r_b = Rect::new(self.top_left_x ,self.top_left_y, DIMENSION as u32, DIMENSION as u32);
        let r_t = Rect::new(self.top_right_x ,self.top_right_y , DIMENSION as u32, DIMENSION as u32);
        let ttl_t = Rect::new(self.top_top_left_x ,self.top_top_left_y , DIMENSION as u32, DIMENSION as u32);
        let rects = [l_b, l_t,r_b, r_t, ttl_t];
        canvas.set_draw_color(color);
        canvas.draw_rects(&rects).ok();
        canvas.fill_rects(&rects).ok();
    }

    fn stop_drag(&mut self) {
        self.is_dragging = false;
        self.offset_x = 0;
        self.offset_y = 0;
    }

    fn reposition(&mut self, x: i32, y: i32) {
        self.bottom_right_x = x + DIMENSION;
        self.bottom_right_y = y;
        self.top_left_x = x;
        self.top_left_y = y - DIMENSION;
        self.top_right_x = x + DIMENSION;
        self.top_right_y = y - DIMENSION;
        self.top_top_left_x = x;
        self.top_top_left_y = y - (2*DIMENSION);
    }

    fn handle_events(&mut self, event: &Event) {
        match *event {
            Event::Quit {..}| Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                process::exit (0x0f00);
            },
            Event::MouseButtonDown{x,y, mouse_btn, ..} => {
                if mouse_btn == MouseButton::Left && self.intersects(self.bottom_left_x,self.bottom_left_y, DIMENSION,x, y)
                                                || self.intersects(self.bottom_right_x, self.bottom_right_y, DIMENSION, x, y)
                                                || self.intersects(self.top_left_x, self.top_left_y, DIMENSION, x, y)
                                                || self.intersects(self.top_right_x, self.top_right_y, DIMENSION, x, y)
                                                || self.intersects(self.top_top_left_x, self.top_top_left_y, DIMENSION, x, y){
                            self.offset_x = x - self.bottom_left_x;
                            self.offset_y = y - self.bottom_left_y;
                            self.is_dragging = true;
                        }

            },
            Event::MouseButtonUp{mouse_btn,..} => {
                if mouse_btn == MouseButton::Left && self.is_dragging {
                    self.stop_drag();
                    if  self.bottom_left_x + DIMENSION > DIMENSION && self.bottom_left_y- 300 < DIMENSION * (MATRIX_SIZE+1) &&
                        self.top_top_left_y < DIMENSION * (MATRIX_SIZE+1) && self.bottom_right_x< DIMENSION * (MATRIX_SIZE+1) {
                            self.bottom_left_x = (self.bottom_left_x / DIMENSION) * DIMENSION;
                            self.bottom_left_y = (self.bottom_left_y / DIMENSION) * DIMENSION;
                            let x = self.bottom_left_x;
                            let y = self.bottom_left_y;
                            self.reposition(x,y);
                            self.check_win(&[x, y]);
                        }
                }
            },
            Event::MouseMotion{x, y,..} => {
                if self.is_dragging {
                    self.bottom_left_x = x - self.offset_x;
                    self.bottom_left_y = y - self.offset_y;
                    let x = self.bottom_left_x;
                    let y = self.bottom_left_y;
                    self.reposition(x,y);
                }
            },
            Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                self.shuffle();
            }
            _ => {}
        }
    }
}

pub struct LShape {
    pub is_dragging: bool,
    pub offset_x: i32,
    pub offset_y: i32,

    pub bottom_left_x: i32,
    pub bottom_left_y: i32,

    pub bottom_right_x: i32,
    pub bottom_right_y: i32,

    pub top_left_x: i32,
    pub top_left_y: i32,

    pub top_right_x: i32,
    pub top_right_y: i32,
}

impl Piece for LShape {
    fn check_win(&self, coords: &[i32;2]) -> bool{
       if self.bottom_left_x == coords[0] && self.bottom_left_y == coords[1] {
           return true
       }
       false
    }
    fn shuffle(&mut self) {
         self.bottom_left_x = rand::thread_rng().gen_range(700, WIDTH as i32 - (2*DIMENSION));
         self.bottom_left_y = rand::thread_rng().gen_range(DIMENSION, HEIGHT as i32 - (2*DIMENSION));
         let x = self.bottom_left_x;
         let y = self.bottom_left_y;
         self.reposition(x,y);
    }
    fn new(x:i32, y: i32) -> LShape {
        let bottom_left_x = x;
        let bottom_left_y = y;
        let bottom_right_x = bottom_left_x + DIMENSION;
        let bottom_right_y = bottom_left_y;

        let top_left_x = bottom_left_x + (2*DIMENSION);
        let top_left_y = bottom_left_y;

        let top_right_x = bottom_left_x + (2*DIMENSION);
        let top_right_y = bottom_left_y - DIMENSION;

        LShape {
            is_dragging: false,
            offset_x: 0,
            offset_y: 0,
            bottom_left_x: bottom_left_x,
            bottom_left_y: bottom_left_y,
            bottom_right_x: bottom_right_x,
            bottom_right_y: bottom_right_y,
            top_left_x: top_left_x,
            top_left_y: top_left_y,
            top_right_x: top_right_x,
            top_right_y: top_right_y,
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>, color: Color) {
        let l_b = Rect::new(self.bottom_left_x,self.bottom_left_y, DIMENSION as u32 , DIMENSION as u32 );
        let l_t = Rect::new(self.bottom_right_x, self.bottom_right_y, DIMENSION as u32, DIMENSION as u32);
        let r_b = Rect::new(self.top_left_x ,self.top_left_y, DIMENSION as u32, DIMENSION as u32);
        let r_t = Rect::new(self.top_right_x ,self.top_right_y , DIMENSION as u32, DIMENSION as u32);
        let rects = [l_b, l_t,r_b, r_t];
        canvas.set_draw_color(color);
        canvas.draw_rects(&rects).ok();
        canvas.fill_rects(&rects).ok();
    }

    fn stop_drag(&mut self) {
        self.is_dragging = false;
        self.offset_x = 0;
        self.offset_y = 0;
    }

    fn reposition(&mut self, x: i32, y: i32) {
        self.bottom_right_x = x + DIMENSION;
        self.bottom_right_y = y;

        self.top_left_x = x + (2*DIMENSION);
        self.top_left_y = y;

        self.top_right_x = x + (2*DIMENSION);
        self.top_right_y = y - DIMENSION;
    }

    fn handle_events(&mut self, event: &Event) {
        match *event {
            Event::Quit {..}| Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                process::exit (0x0f00);
            },
            Event::MouseButtonDown{x,y, mouse_btn, ..} => {
                if mouse_btn == MouseButton::Left && self.intersects(self.bottom_left_x,self.bottom_left_y, DIMENSION,x, y)
                                                || self.intersects(self.bottom_right_x, self.bottom_right_y, DIMENSION, x, y)
                                                || self.intersects(self.top_left_x, self.top_left_y, DIMENSION, x, y)
                                                || self.intersects(self.top_right_x, self.top_right_y, DIMENSION, x, y){
                        self.offset_x = x - self.bottom_left_x;
                        self.offset_y = y - self.bottom_left_y;
                        self.is_dragging = true;
                }
            },
            Event::MouseButtonUp{mouse_btn,..} => {
                if mouse_btn == MouseButton::Left && self.is_dragging {
                    self.stop_drag();
                    if  self.bottom_left_x + DIMENSION > DIMENSION && self.bottom_right_x < DIMENSION * (MATRIX_SIZE+1) &&
                        self.top_left_y > DIMENSION && self.top_left_y < DIMENSION * (MATRIX_SIZE+1) {
                            self.bottom_left_x = (self.bottom_left_x / DIMENSION) * DIMENSION;
                            self.bottom_left_y = (self.bottom_left_y / DIMENSION) * DIMENSION;
                            let x = self.bottom_left_x;
                            let y = self.bottom_left_y;
                            self.reposition(x,y);
                            self.check_win(&[x, y]);
                        }
                }
            },
            Event::MouseMotion{x, y,..} => {
                if self.is_dragging {
                    self.bottom_left_x = x - self.offset_x;
                    self.bottom_left_y = y - self.offset_y;
                    let x = self.bottom_left_x;
                    let y = self.bottom_left_y;
                    self.reposition(x,y);
                }
            },
            Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                self.shuffle();
            }
            _ => {}
        }
    }
}

pub struct RShape {
    pub is_dragging: bool,
    pub offset_x: i32,
    pub offset_y: i32,

    pub bottom_left_x: i32,
    pub bottom_left_y: i32,

    pub bottom_right_x: i32,
    pub bottom_right_y: i32,

    pub top_left_x: i32,
    pub top_left_y: i32,

    pub top_right_x: i32,
    pub top_right_y: i32,
}

impl  Piece for RShape {
    fn check_win(&self, coords: &[i32;2]) -> bool{
       if self.bottom_left_x == coords[0] && self.bottom_left_y == coords[1] {
           return true;
       }
       false
    }
    fn shuffle(&mut self) {
         self.bottom_left_x = rand::thread_rng().gen_range(700, WIDTH as i32 - (2*DIMENSION));
         self.bottom_left_y = rand::thread_rng().gen_range(DIMENSION, HEIGHT as i32 - (2*DIMENSION));
         let x = self.bottom_left_x;
         let y = self.bottom_left_y;
         self.reposition(x,y);
    }
    fn new(x:i32, y: i32) -> RShape {
        let bottom_left_x = x;
        let bottom_left_y = y;
        let bottom_right_x = bottom_left_x + DIMENSION;
        let bottom_right_y = bottom_left_y;

        let top_left_x = bottom_left_x + (2*DIMENSION);
        let top_left_y = bottom_left_y;

        let top_right_x = bottom_left_x;
        let top_right_y = bottom_left_y + DIMENSION;

        RShape {
            is_dragging: false,
            offset_x: 0,
            offset_y: 0,
            bottom_left_x: bottom_left_x,
            bottom_left_y: bottom_left_y,
            bottom_right_x: bottom_right_x,
            bottom_right_y: bottom_right_y,
            top_left_x: top_left_x,
            top_left_y: top_left_y,
            top_right_x: top_right_x,
            top_right_y: top_right_y,
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>, color: Color) {
        let l_b = Rect::new(self.bottom_left_x,self.bottom_left_y, DIMENSION as u32 , DIMENSION as u32 );
        let l_t = Rect::new(self.bottom_right_x, self.bottom_right_y, DIMENSION as u32, DIMENSION as u32);
        let r_b = Rect::new(self.top_left_x ,self.top_left_y, DIMENSION as u32, DIMENSION as u32);
        let r_t = Rect::new(self.top_right_x ,self.top_right_y , DIMENSION as u32, DIMENSION as u32);
        let rects = [l_b, l_t,r_b, r_t];
        canvas.set_draw_color(color);
        canvas.draw_rects(&rects).ok();
        canvas.fill_rects(&rects).ok();
    }

    fn stop_drag(&mut self) {
        self.is_dragging = false;
        self.offset_x = 0;
        self.offset_y = 0;
    }

    fn reposition(&mut self, x: i32, y: i32) {
        self.bottom_right_x = x + DIMENSION;
        self.bottom_right_y = y;
        self.top_left_x = x + (2*DIMENSION);
        self.top_left_y = y;
        self.top_right_x = x;
        self.top_right_y = y + DIMENSION;
    }

    fn handle_events(&mut self, event: &Event) {
        match *event {
            Event::Quit {..}| Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                process::exit (0x0f00);
            },
            Event::MouseButtonDown{x,y, mouse_btn, ..} => {
                if mouse_btn == MouseButton::Left && self.intersects(self.bottom_left_x,self.bottom_left_y, DIMENSION,x, y)
                                                || self.intersects(self.bottom_right_x, self.bottom_right_y, DIMENSION, x, y)
                                                || self.intersects(self.top_left_x, self.top_left_y, DIMENSION, x, y)
                                                || self.intersects(self.top_right_x, self.top_right_y, DIMENSION, x, y){
                            self.offset_x = x - self.bottom_left_x;
                            self.offset_y = y - self.bottom_left_y;
                            self.is_dragging = true;
                        }

            },
            Event::MouseButtonUp{mouse_btn,..} => {
                if mouse_btn == MouseButton::Left && self.is_dragging {
                    self.stop_drag();
                    if  self.bottom_left_x + DIMENSION > DIMENSION && self.bottom_right_x < DIMENSION * (MATRIX_SIZE+1) &&
                        self.bottom_left_y > DIMENSION && self.top_right_y < DIMENSION * (MATRIX_SIZE+1) {
                            self.bottom_left_x = (self.bottom_left_x / DIMENSION) * DIMENSION;
                            self.bottom_left_y = (self.bottom_left_y / DIMENSION) * DIMENSION;
                            let x = self.bottom_left_x;
                            let y = self.bottom_left_y;
                            self.reposition(x, y);
                            self.check_win(&[x, y]);
                        }
                }
            },
            Event::MouseMotion{x, y,..} => {
                if self.is_dragging {
                    self.bottom_left_x = x - self.offset_x;
                    self.bottom_left_y = y - self.offset_y;
                    let x = self.bottom_left_x;
                    let y = self.bottom_left_y;
                    self.reposition(x, y);
                }
            },
            Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                self.shuffle();
            }
            _ => {}
        }
    }
}
