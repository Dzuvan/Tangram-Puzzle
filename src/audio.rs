extern crate sdl2;

use std::time::Duration;
use std::path::Path;
use std::thread;

use sdl2::mixer::{DEFAULT_CHANNELS, INIT_MP3, INIT_FLAC, INIT_MOD, INIT_FLUIDSYNTH, INIT_MODPLUG,
                  INIT_OGG, AUDIO_S16LSB};

pub fn play(music_file: &Path, time: i32) {

    println!("linked version: {}", sdl2::mixer::get_linked_version());

    let _mixer_context = sdl2::mixer::init(INIT_MP3 | INIT_FLAC | INIT_MOD | INIT_FLUIDSYNTH |
                                           INIT_MODPLUG | INIT_OGG)
            .unwrap();

    let frequency = 44100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();

    // Number of mixing channels available for sound effect `Chunk`s to play
    // simultaneously.
    sdl2::mixer::allocate_channels(4);

    {
        let n = sdl2::mixer::get_chunk_decoders_number();
        println!("available chunk(sample) decoders: {}", n);
        for i in 0..n {
            println!("  decoder {} => {}", i, sdl2::mixer::get_chunk_decoder(i));
        }
    }

    {
        let n = sdl2::mixer::get_music_decoders_number();
        println!("available music decoders: {}", n);
        for i in 0..n {
            println!("  decoder {} => {}", i, sdl2::mixer::get_music_decoder(i));
        }
    }

    println!("query spec => {:?}", sdl2::mixer::query_spec());

    let music = sdl2::mixer::Music::from_file(music_file).unwrap();

    fn hook_finished() {
        println!("play ends! from rust cb");
    }

    sdl2::mixer::Music::hook_finished(hook_finished);

     println!("music => {:?}", music);
    // println!("music type => {:?}", music.get_type());
    // println!("music volume => {:?}", sdl2::mixer::Music::get_volume());
     println!("play => {:?}", music.play(time));

    println!("fading out ... {:?}", sdl2::mixer::Music::fade_out(time));

    // println!("fading in from pos ... {:?}",
    //          music.fade_in_from_pos(1, 10000, 100.0));
    thread::sleep(Duration::from_millis(time as u64));
    sdl2::mixer::Music::halt();
}

