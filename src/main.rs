extern crate rustc_serialize;
extern crate docopt;

extern crate libc;

#[macro_use]
extern crate enum_primitive;
extern crate num;

extern crate gl;
extern crate sdl2;
extern crate sdl2_sys;


use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2_sys::video::SDL_GL_SwapWindow;

use std::ffi::CStr;
use std::mem;


mod mpv;
mod mpv_gen;

const USAGE: &'static str = "
toyunda-player.

Usage:
  toyunda-player [options] <file>
  toyunda-player -h | --help
  toyunda-player --version

Options:
  -h --help     Show this screen.
  --version     Show version.
  --invert      Invert the screen.
";

#[derive(Debug, RustcDecodable)]
struct CmdArgs {
    flag_invert: bool,
    arg_file: String
}

unsafe extern "C" fn do_pote(arg: *mut std::os::raw::c_void, name: *const std::os::raw::c_char) -> *mut std::os::raw::c_void {
    let arg: &sdl2::VideoSubsystem = mem::transmute(arg);
    let name = CStr::from_ptr(name).to_str().unwrap();
    arg.gl_get_proc_address(name) as *mut std::os::raw::c_void
}

fn get_mpv_gl(mpv: &mpv::Mpv, video_subsystem: &sdl2::VideoSubsystem) -> mpv::OpenglContext {
    mpv.get_opengl_context(Some(do_pote), unsafe {mem::transmute(video_subsystem)}).unwrap()
}

fn main() {
    let args: CmdArgs = docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    video_subsystem.gl_load_library_default().unwrap();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .resizable()
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    renderer.clear();
    renderer.present();

    let mpv = mpv::Mpv::init().unwrap();
    let mpv_gl = get_mpv_gl(&mpv, &video_subsystem);
    mpv.set_option("vo", "opengl-cb");
    mpv.command(&["loadfile", &args.arg_file as &str]).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            let set_prop_s = |p,v| mpv.set_property_string(p,v) ;
            let set_prop_f = |p,v| mpv.set_property_float(p,v) ;
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    match mpv.get_property_string("pause") {
                        "yes" => {set_prop_s("pause","no").unwrap();},
                        "no" => {set_prop_s("pause","yes").unwrap();},
                        _ => {panic!("unexpected answer from get_property_string");}
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Kp9), .. } => {set_prop_f("speed",0.9).unwrap();},
                Event::KeyDown { keycode: Some(Keycode::Kp8), .. } => {set_prop_f("speed",0.8).unwrap();},
                Event::KeyDown { keycode: Some(Keycode::Kp7), .. } => {set_prop_f("speed",0.7).unwrap();},
                Event::KeyDown { keycode: Some(Keycode::Kp6), .. } => {set_prop_f("speed",0.6).unwrap();},
                Event::KeyDown { keycode: Some(Keycode::Kp5), .. } => {set_prop_f("speed",0.5).unwrap();},
                Event::KeyDown { keycode: Some(Keycode::Kp4), .. } => {set_prop_f("speed",0.4).unwrap();},
                Event::KeyDown { keycode: Some(Keycode::Kp3), .. } => {set_prop_f("speed",0.3).unwrap();},
                Event::KeyDown { keycode: Some(Keycode::Kp2), .. } => {set_prop_f("speed",0.2).unwrap();},
                Event::KeyDown { keycode: Some(Keycode::Kp1), .. } => {set_prop_f("speed",0.1).unwrap();},
                Event::KeyDown { keycode: Some(Keycode::Kp0), .. } => {set_prop_f("speed",1.0).unwrap();},
                _ => {}
            }
        }
        while let Some(event) = mpv.wait_event() {
            // do something with the events
            // but it's kind of useless
            // it's still necessary to empty the event pool
        }
        let (width, height) = renderer.window().unwrap().size();
        mpv_gl.draw(0, width as i32, -(height as i32));
        unsafe {
            SDL_GL_SwapWindow(renderer.window().unwrap().raw());
        }
    }
}
