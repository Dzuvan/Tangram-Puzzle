extern crate sdl2;
extern crate rand;

use std::path::Path;

pub mod shapes;
pub mod constants;
pub mod board;

use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::render::{ Canvas, TextureQuery };
use sdl2::rect::Rect;

use constants::*;
use shapes::*;
use board::*;

fn init_sdl() ->  (Canvas<Window>, sdl2::EventPump) {
    let sdl_context = sdl2::init ().ok ().expect ("Could not initialize SDL2");
    let video_subsystem  = sdl_context.video ().ok ().expect ("Could not init video_subsystem");

    let window = video_subsystem.window ("Game", WIDTH, HEIGHT)
        .position_centered ()
        .opengl ()
        .build ()
        .unwrap ();

    let  canvas = window.into_canvas ()
        .present_vsync ()
        .build ()
        .unwrap ();

    let event_pump = sdl_context.event_pump ().unwrap ();
    (canvas, event_pump)
}

fn main() {
    let (mut canvas, mut event_pump) = init_sdl ();

    let ttf_context = sdl2::ttf::init().unwrap();
    let mut font = ttf_context.load_font(Path::new("assets/font.ttf"), 128).unwrap();
    let texture_creator = canvas.texture_creator();
    let surface = font.render("Hello Rust!") .blended(Color::RGBA(255, 255, 255, 1)).unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    let TextureQuery { width, height, .. } = texture.query();

    font.set_style(sdl2::ttf::STYLE_BOLD);
    let padding = 64;
    let target = get_centered_rect(width, height, WIDTH - padding, HEIGHT - padding);

    let board = Board::new();

    let mut s = SShape::new(100, 300);
    let mut u = UShape::new(400, 300);
    let mut i = IShape::new(500, 300);
    let mut g = GShape::new(300, 300);
    let mut f = FShape::new(100, 300);
    let mut l = LShape::new(200, 200);
    let mut r = RShape::new(100, 400);

    let solution_1 = vec![[300, 500], [200, 300], [100, 100], [500, 500], [100, 500],[300, 200],[200, 100]];
    let solution_2 = vec![[300, 500], [200, 300], [500, 100], [500, 500], [100, 500],[200, 200],[100, 100]];
    let solution_3 = vec![[100, 300], [100, 100], [100, 400], [500, 300], [300, 300],[300, 500],[200, 400]];
    let solution_4 = vec![[100, 200], [100, 300], [100, 400], [500, 300], [300, 300],[300, 500],[200, 400]];
    let solution_5 = vec![[100, 200], [100, 300], [500, 400], [500, 300], [300, 300],[200, 500],[100, 400]];
    let solution_6 = vec![[100, 300], [100, 100], [500, 400], [500, 300], [300, 300],[200, 500],[100, 400]];
    let solution_7 = vec![[400, 500], [400, 300], [500, 200], [300, 500], [100, 500],[200, 200],[100, 100]];
    let solution_8 = vec![[400, 400], [400, 500], [500, 200], [300, 500], [100, 500],[200, 200],[100, 100]];
    let solution_9 = vec![[400, 200], [400, 300], [100, 500], [300, 300], [100, 300],[300, 500],[200, 400]];
    let solution_10 = vec![[300, 300], [200, 100], [500, 500], [500, 300], [100, 300],[200, 500],[100, 400]];
    let solution_11 = vec![[400, 200], [400, 300], [500, 400], [300, 300], [100, 300],[200, 500],[100, 400]];
    let solution_12 = vec![[300, 300], [300, 400], [200, 200], [500, 300], [100, 500],[300, 500],[100, 100]];
    let solution_13 = vec![[200, 300], [300, 400], [400, 200], [500, 300], [100, 500],[300, 500],[100, 100]];
    let solution_14 = vec![[300, 400], [300, 200], [200, 200], [500, 300], [100, 500],[300, 500],[100, 100]];
    let solution_15 = vec![[300, 500], [200, 200], [500, 400], [500, 300], [100, 500],[200, 300],[100, 100]];
    let solution_16 = vec![[400, 500], [200, 200], [300, 400], [500, 300], [100, 500],[200, 300],[100, 100]];
      // s.shuffle();
      // u.shuffle();
      // i.shuffle();
      // g.shuffle();
      // f.shuffle();
      // l.shuffle();
      // r.shuffle();

    loop {

        for event in event_pump.poll_iter() {
            s.handle_events(&event);
            u.handle_events(&event);
            i.handle_events(&event);
            g.handle_events(&event);
            f.handle_events(&event);
            l.handle_events(&event);
            r.handle_events(&event);

            if s.check_win(&solution_1[0])&&
            u.check_win(&solution_1[1])&&
            i.check_win(&solution_1[2])&&
            g.check_win(&solution_1[3])&&
            f.check_win(&solution_1[4])&&
            l.check_win(&solution_1[5])&&
            r.check_win(&solution_1[6]){
              board.end_screen(&mut s,&mut u,&mut i,&mut g,&mut f,&mut l,&mut r);
            }

            if s.check_win(&solution_2[0])&&
            u.check_win(&solution_2[1])&&
            i.check_win(&solution_2[2])&&
            g.check_win(&solution_2[3])&&
            f.check_win(&solution_2[4])&&
            l.check_win(&solution_2[5])&&
            r.check_win(&solution_2[6]){
              board.end_screen(&mut s,&mut u,&mut i,&mut g,&mut f,&mut l,&mut r);
            }
            if s.check_win(&solution_3[0])&&
            u.check_win(&solution_3[1])&&
            i.check_win(&solution_3[2])&&
            g.check_win(&solution_3[3])&&
            f.check_win(&solution_3[4])&&
            l.check_win(&solution_3[5])&&
            r.check_win(&solution_3[6]){
              board.end_screen(&mut s,&mut u,&mut i,&mut g,&mut f,&mut l,&mut r);
            }

            if s.check_win(&solution_4[0])&&
            u.check_win(&solution_4[1])&&
            i.check_win(&solution_4[2])&&
            g.check_win(&solution_4[3])&&
            f.check_win(&solution_4[4])&&
            l.check_win(&solution_4[5])&&
            r.check_win(&solution_4[6]){
              board.end_screen(&mut s,&mut u,&mut i,&mut g,&mut f,&mut l,&mut r);
            }

            if s.check_win(&solution_5[0])&&
            u.check_win(&solution_5[1])&&
            i.check_win(&solution_5[2])&&
            g.check_win(&solution_5[3])&&
            f.check_win(&solution_5[4])&&
            l.check_win(&solution_5[5])&&
            r.check_win(&solution_5[6]){
              board.end_screen(&mut s,&mut u,&mut i,&mut g,&mut f,&mut l,&mut r);
            }

            if s.check_win(&solution_6[0])&&
            u.check_win(&solution_6[1])&&
            i.check_win(&solution_6[2])&&
            g.check_win(&solution_6[3])&&
            f.check_win(&solution_6[4])&&
            l.check_win(&solution_6[5])&&
            r.check_win(&solution_6[6]){
              board.end_screen(&mut s,&mut u,&mut i,&mut g,&mut f,&mut l,&mut r);
            }

            if s.check_win(&solution_7[0])&&
            u.check_win(&solution_7[1])&&
            i.check_win(&solution_7[2])&&
            g.check_win(&solution_7[3])&&
            f.check_win(&solution_7[4])&&
            l.check_win(&solution_7[5])&&
            r.check_win(&solution_7[6]){
              board.end_screen(&mut s,&mut u,&mut i,&mut g,&mut f,&mut l,&mut r);
            }

            if s.check_win(&solution_8[0])&&
            u.check_win(&solution_8[1])&&
            i.check_win(&solution_8[2])&&
            g.check_win(&solution_8[3])&&
            f.check_win(&solution_8[4])&&
            l.check_win(&solution_8[5])&&
            r.check_win(&solution_8[6]){
              board.end_screen(&mut s,&mut u,&mut i,&mut g,&mut f,&mut l,&mut r);
            }

            if s.check_win(&solution_9[0])&&
            u.check_win(&solution_9[1])&&
            i.check_win(&solution_9[2])&&
            g.check_win(&solution_9[3])&&
            f.check_win(&solution_9[4])&&
            l.check_win(&solution_9[5])&&
            r.check_win(&solution_9[6]){
              board.end_screen(&mut s,&mut u,&mut i,&mut g,&mut f,&mut l,&mut r);
            }

            if s.check_win(&solution_10[0])&&
            u.check_win(&solution_10[1])&&
            i.check_win(&solution_10[2])&&
            g.check_win(&solution_10[3])&&
            f.check_win(&solution_10[4])&&
            l.check_win(&solution_10[5])&&
            r.check_win(&solution_10[6]){
              board.end_screen(&mut s,&mut u,&mut i,&mut g,&mut f,&mut l,&mut r);
            }

            if s.check_win(&solution_11[0])&&
            u.check_win(&solution_11[1])&&
            i.check_win(&solution_11[2])&&
            g.check_win(&solution_11[3])&&
            f.check_win(&solution_11[4])&&
            l.check_win(&solution_11[5])&&
            r.check_win(&solution_11[6]){
              board.end_screen(&mut s,&mut u,&mut i,&mut g,&mut f,&mut l,&mut r);
            }

            if s.check_win(&solution_12[0])&&
            u.check_win(&solution_12[1])&&
            i.check_win(&solution_12[2])&&
            g.check_win(&solution_12[3])&&
            f.check_win(&solution_12[4])&&
            l.check_win(&solution_12[5])&&
            r.check_win(&solution_12[6]){
              board.end_screen(&mut s,&mut u,&mut i,&mut g,&mut f,&mut l,&mut r);
            }

            if s.check_win(&solution_13[0])&&
            u.check_win(&solution_13[1])&&
            i.check_win(&solution_13[2])&&
            g.check_win(&solution_13[3])&&
            f.check_win(&solution_13[4])&&
            l.check_win(&solution_13[5])&&
            r.check_win(&solution_13[6]){
              board.end_screen(&mut s,&mut u,&mut i,&mut g,&mut f,&mut l,&mut r);
            }

            if s.check_win(&solution_14[0])&&
            u.check_win(&solution_14[1])&&
            i.check_win(&solution_14[2])&&
            g.check_win(&solution_14[3])&&
            f.check_win(&solution_14[4])&&
            l.check_win(&solution_14[5])&&
            r.check_win(&solution_14[6]){
              board.end_screen(&mut s,&mut u,&mut i,&mut g,&mut f,&mut l,&mut r);
            }

            if s.check_win(&solution_15[0])&&
            u.check_win(&solution_15[1])&&
            i.check_win(&solution_15[2])&&
            g.check_win(&solution_15[3])&&
            f.check_win(&solution_15[4])&&
            l.check_win(&solution_15[5])&&
            r.check_win(&solution_15[6]){
              board.end_screen(&mut s,&mut u,&mut i,&mut g,&mut f,&mut l,&mut r);
            }

            if s.check_win(&solution_16[0])&&
            u.check_win(&solution_16[1])&&
            i.check_win(&solution_16[2])&&
            g.check_win(&solution_16[3])&&
            f.check_win(&solution_16[4])&&
            l.check_win(&solution_16[5])&&
            r.check_win(&solution_16[6]){
              board.end_screen(&mut s,&mut u,&mut i,&mut g,&mut f,&mut l,&mut r);
            }

        }
        // Boja pozadine.
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.copy(&texture, None, Some(target)).unwrap();
        board.draw_grid(&mut canvas, Color::RGB(255, 0, 0));
        s.draw(&mut canvas,Color::RGB(255, 0, 255));
        u.draw(&mut canvas,Color::RGB(0, 0, 255));
        i.draw(&mut canvas,Color::RGB(0, 255, 0));
        g.draw(&mut canvas,Color::RGB(255, 255, 0));
        f.draw(&mut canvas,Color::RGB(0, 255, 255));
        l.draw(&mut canvas,Color::RGB(100, 255, 255));
        r.draw(&mut canvas,Color::RGB(100, 0, 255));
        canvas.present();
    }
}

fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };
    let cx = (WIDTH as i32 - w) / 2;
    let cy = (HEIGHT as i32 - h) / 2;

    Rect::new(cx, cy, w as u32, h as u32)
}

