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
    println!("{:?}", args);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    video_subsystem.gl_load_library_default().unwrap();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    renderer.set_draw_color(Color::RGB(255, 0, 0));
    renderer.clear();
    renderer.present();

    let mpv = mpv::Mpv::init().unwrap();
    let mpv_gl = get_mpv_gl(&mpv, &video_subsystem);
    mpv.set_option("vo", "opengl-cb");
    mpv.command(&["loadfile", &args.arg_file as &str]).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        mpv_gl.draw(0, 800, 600);
        unsafe {
            SDL_GL_SwapWindow(renderer.window().unwrap().raw());
        }
    }
}
