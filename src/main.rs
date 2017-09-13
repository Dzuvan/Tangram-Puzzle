extern crate sdl2;
extern crate rand;

use std::path::Path;
use std::time::Duration;
use std::borrow::Cow;

pub mod shapes;
pub mod constants;
pub mod board;

use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::audio::{ AudioDevice, AudioCallback, AudioSpecDesired,AudioSpecWAV,AudioCVT };
use sdl2::render::{ Canvas, TextureQuery };
use sdl2::rect::Rect;
use sdl2::AudioSubsystem;

use constants::*;
use shapes::*;
use board::*;

fn init_sdl() ->  (Canvas<Window>, sdl2::EventPump, AudioSubsystem) {
    let sdl_context = sdl2::init ().ok ().expect ("Could not initialize SDL2");
    let video_subsystem  = sdl_context.video ().ok ().expect ("Could not init video_subsystem");
    let audio_subsystem = sdl_context.audio().unwrap();


    let window = video_subsystem.window ("Puzle", WIDTH, HEIGHT)
        .position_centered ()
        .opengl ()
        .build ()
        .unwrap ();

    let  canvas = window.into_canvas ()
        .present_vsync ()
        .build ()
        .unwrap ();

    let event_pump = sdl_context.event_pump ().unwrap ();
    (canvas, event_pump, audio_subsystem)
}

fn main() {
    let (mut canvas, mut event_pump, audio_subsystem) = init_sdl ();

    let ttf_context = sdl2::ttf::init().unwrap();
    let mut font = ttf_context.load_font(Path::new("assets/font.ttf"), 32).unwrap();
    let texture_creator = canvas.texture_creator();
    let surface = font.render("Tangram Puzzle") .blended(Color::RGBA(255, 255, 255, 1)).unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    let TextureQuery { width, height, .. } = texture.query();

    font.set_style(sdl2::ttf::STYLE_BOLD);
    println!("{}, {}", width, height);
    let target = Rect::new(3*DIMENSION, DIMENSION/3, WIDTH/2,HEIGHT/14);

    let help = font.render("Help:\nPress R to restart.\nPress ESC to exit.") .blended(Color::RGBA(255, 255, 255, 1)).unwrap();
    let help_texture = texture_creator.create_texture_from_surface(&help).unwrap();
    let help_target = Rect::new(DIMENSION as i32, 6 * DIMENSION as i32, WIDTH / 2, HEIGHT / 12);

    let wav_file : Cow<'static, Path> = Cow::from(Path::new("assets/captcha.wav"));
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1), // mono
        samples: None      // default
    };
    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        let wav = AudioSpecWAV::load_wav(wav_file)
            .expect("Could not load test WAV file");

        let cvt = AudioCVT::new(
                wav.format, wav.channels, wav.freq,
                spec.format, spec.channels, spec.freq)
            .expect("Could not convert WAV file");

        let data = cvt.convert(wav.buffer().to_vec());

        // initialize the audio callback
        Sound {
            data: data,
            volume: 0.25,
            pos: 0,
        }
    }).unwrap();

    let board = Board::new();

    let mut s = SShape::new(400, 400);
    let mut u = UShape::new(400, 500);
    let mut i = IShape::new(100, 100);
    let mut g = GShape::new(300, 500);
    let mut f = FShape::new(100, 500);
    let mut l = LShape::new(200, 100);
    let mut r = RShape::new(300, 200);

    let solution_1 = vec![[300, 500], [200, 300], [100, 100], [500, 500], [100, 500],[300, 200],[200, 100]];
    let solution_2 = vec![[300, 500], [200, 300], [500, 100], [500, 500], [100, 500],[200, 200],[100, 100]];
    let solution_3 = vec![[100, 300], [100, 100], [100, 400], [500, 300], [300, 300],[300, 500],[200, 400]];
    let solution_4 = vec![[100, 200], [100, 300], [100, 400], [500, 300], [300, 300],[300, 500],[200, 400]];
    let solution_5 = vec![[100, 200], [100, 300], [500, 400], [500, 300], [300, 300],[200, 500],[100, 400]];
    let solution_6 = vec![[100, 300], [100, 100], [500, 400], [500, 300], [300, 300],[200, 500],[100, 400]];
    let solution_9 = vec![[400, 200], [400, 300], [100, 500], [300, 300], [100, 300],[300, 500],[200, 400]];
    let solution_10 = vec![[300, 300], [200, 100], [500, 500], [500, 300], [100, 300],[200, 500],[100, 400]];
    let solution_11 = vec![[400, 200], [400, 300], [500, 400], [300, 300], [100, 300],[200, 500],[100, 400]];
    let solution_12 = vec![[300, 300], [300, 400], [200, 200], [500, 300], [100, 500],[300, 500],[100, 100]];
    let solution_13 = vec![[200, 300], [300, 400], [400, 200], [500, 300], [100, 500],[300, 500],[100, 100]];
    let solution_14 = vec![[300, 400], [300, 200], [200, 200], [500, 300], [100, 500],[300, 500],[100, 100]];
    let solution_15 = vec![[300, 500], [200, 200], [500, 400], [500, 300], [100, 500],[200, 300],[100, 100]];
    let solution_16 = vec![[400, 500], [200, 200], [300, 400], [500, 300], [100, 500],[200, 300],[100, 100]];
    let solution_17 = vec![[100, 400], [100, 500], [100, 100], [500, 500], [300, 500],[300, 200],[200, 100]];
    let solution_18 = vec![[300, 400], [200, 100], [100, 100], [500, 300], [100, 500],[300, 500],[200, 200]];
    let solution_19 = vec![[300, 300], [200, 100], [100, 400], [500, 300], [100, 300],[300, 500],[200, 400]];
    let solution_20 = vec![[400, 400], [400, 500], [100, 100], [300, 500], [100, 500],[300, 200],[200, 100]];
    let solution_21 = vec![[400, 500], [400, 300], [100, 100], [300, 500], [100, 500],[300, 200],[200, 100]];
    let solution_22 = vec![[400, 500], [400, 300], [500, 100], [300, 500], [100, 500],[200, 200],[100, 100]];
    let solution_23 = vec![[400, 400], [400, 500], [500, 100], [300, 500], [100, 500],[200, 200],[100, 100]];

     let solutions = vec![solution_1,solution_2,solution_3,solution_4,solution_5,solution_6,solution_9,
     solution_10, solution_11,solution_12,solution_13,solution_14, solution_15, solution_16, solution_17, solution_18, solution_19,
     solution_20, solution_21, solution_22, solution_23];

        s.shuffle();
        u.shuffle();
        i.shuffle();
        g.shuffle();
        f.shuffle();
        l.shuffle();
        r.shuffle();

    loop {

        for event in event_pump.poll_iter() {
            s.handle_events(&event);
            u.handle_events(&event);
            i.handle_events(&event);
            g.handle_events(&event);
            f.handle_events(&event);
            l.handle_events(&event);
            r.handle_events(&event);

        for solution  in &solutions {

            if s.check_win(&solution[0])&&
            u.check_win(&solution[1])&&
            i.check_win(&solution[2])&&
            g.check_win(&solution[3])&&
            f.check_win(&solution[4])&&
            l.check_win(&solution[5])&&
            r.check_win(&solution[6]){
                // Start playback
                device.resume();
              board.end_screen(&mut s,&mut u,&mut i,&mut g,&mut f,&mut l,&mut r);
            }

        }
        }
        // Boja pozadine.
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.copy(&texture, None, Some(target)).unwrap();
        canvas.copy(&help_texture, None, Some(help_target)).unwrap();

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

struct Sound {
    data: Vec<u8>,
    volume: f32,
    pos: usize,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for dst in out.iter_mut() {
            *dst = (*self.data.get(self.pos).unwrap_or(&0) as f32 * self.volume) as u8;
            self.pos += 1;
        }
    }
}




