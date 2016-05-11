extern crate rustc_serialize;
extern crate docopt;

#[macro_use]
extern crate enum_primitive;
extern crate num;

extern crate gl;
extern crate sdl2;
extern crate sdl2_sys;
extern crate sdl2_ttf;

#[macro_use]
extern crate log;
extern crate env_logger;

use gl::types::* ;

use std::time;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::FullscreenType;
use sdl2_sys::video::SDL_WindowFlags;

use std::ffi::CStr;
use std::os::raw as libc;
use std::ops::Deref;

mod displayer ;


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
    arg_file: String,
}

unsafe extern "C" fn do_pote(arg: *mut libc::c_void,
                             name: *const libc::c_char) -> *mut libc::c_void {
    let arg: &sdl2::VideoSubsystem = &*(arg as *mut sdl2::VideoSubsystem);
    let name = CStr::from_ptr(name).to_str().unwrap();
    arg.gl_get_proc_address(name) as *mut libc::c_void
}

fn get_mpv_gl(mpv: &mpv::Mpv, video_subsystem: &mut sdl2::VideoSubsystem) -> mpv::OpenglContext {
    let ptr = video_subsystem as *mut _ as *mut libc::c_void;
    mpv.get_opengl_context(Some(do_pote), ptr).unwrap()
}

fn main() {
    env_logger::init().unwrap();

    let args: CmdArgs = docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let sdl_context = sdl2::init().unwrap();

    let mut video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Toyunda Player", 960, 540)
        .resizable()
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut opengl_driver : Option<i32> = None ;
    info!("Detecting drivers ...");
    let mut driver_index = -1 ;
    for item in sdl2::render::drivers() {
        driver_index = driver_index + 1 ;
        info!("* '{}'",item.name);
        if (item.name == "opengl"){
            info!("Found opengl driver !");
            opengl_driver = Some(driver_index);
        }
    }
    debug!("End of driver detection");
    let opengl_driver = opengl_driver.expect("No opengl driver found, aborting") as u32 ;
    let mut renderer = window.renderer()
        .present_vsync()
        .index(opengl_driver)
        .build()
        .expect("Failed to create renderer with given parameters");
    let mut displayer = displayer::Displayer::new(renderer).expect("Failed to create displayer");
    displayer.sdl_renderer()
        .window()
        .expect("Failed to extract window from displayer")
        .gl_set_context_to_current();

    let mpv = mpv::Mpv::init().expect("Error while initializing MPV");
    let mpv_gl = get_mpv_gl(&mpv, &mut video_subsystem);
    mpv.set_option("vo", "opengl-cb").expect("Error setting vo option to opengl-cb");
    mpv.set_option("sid", "no").expect("Error setting custom option sid to false");
    mpv.command(&["loadfile", &args.arg_file as &str]).expect("Error loading file");

    let mut event_pump = sdl_context.event_pump().expect("Failed to create event_pump");
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space),repeat: false, .. } => {
                    let instant = time::Instant::now() ;
                    match mpv.get_property_string("pause") {
                        "yes" => {mpv.set_property("pause","no").expect("Failed to pause player");},
                        "no" => {mpv.set_property("pause","yes").expect("Failed to unpause player");},
                        _ => {panic!("unexpected answer from get_property_string");}
                    }
                    println!("Duration of pause / unpause : {},{}",instant.elapsed().as_secs(),instant.elapsed().subsec_nanos());
                },
                Event::KeyDown { keycode: Some(Keycode::Kp9), repeat: false, .. } => {mpv.set_property_async("speed",0.9).unwrap();},
                Event::KeyDown { keycode: Some(Keycode::Kp8), repeat: false, .. } => {mpv.set_property_async("speed",0.8).unwrap();},
                Event::KeyDown { keycode: Some(Keycode::Kp7), repeat: false, .. } => {mpv.set_property_async("speed",0.7).unwrap();},
                Event::KeyDown { keycode: Some(Keycode::Kp6), repeat: false, .. } => {mpv.set_property_async("speed",0.6).unwrap();},
                Event::KeyDown { keycode: Some(Keycode::Kp5), repeat: false, .. } => {mpv.set_property_async("speed",0.5).unwrap();},
                Event::KeyDown { keycode: Some(Keycode::Kp4), repeat: false, .. } => {mpv.set_property_async("speed",0.4).unwrap();},
                Event::KeyDown { keycode: Some(Keycode::Kp3), repeat: false, .. } => {mpv.set_property_async("speed",0.3).unwrap();},
                Event::KeyDown { keycode: Some(Keycode::Kp2), repeat: false, .. } => {mpv.set_property_async("speed",0.2).unwrap();},
                Event::KeyDown { keycode: Some(Keycode::Kp1), repeat: false, .. } => {mpv.set_property_async("speed",0.1).unwrap();},
                Event::KeyDown { keycode: Some(Keycode::Kp0), repeat: false, .. } => {mpv.set_property_async("speed",1.0).unwrap();},
                Event::KeyDown { keycode: Some(Keycode::F), repeat: false, .. } => {
                    let instant = time::Instant::now() ;
                    if (displayer.sdl_renderer().window().unwrap().window_flags() &
                        (SDL_WindowFlags::SDL_WINDOW_FULLSCREEN as u32)) != 0 {
                        displayer.sdl_renderer_mut().window_mut().unwrap().set_fullscreen(FullscreenType::Off)
                    } else {
                        displayer.sdl_renderer_mut().window_mut().unwrap().set_fullscreen(FullscreenType::Desktop)
                    }
                    .expect("Failed to change fullscreen parameter of mpv");
                    println!("Duration of fullscreen : {},{}",instant.elapsed().as_secs(),instant.elapsed().subsec_nanos());
                }
                _ => {}
            }
        }
        while let Some(_) = mpv.wait_event() {
            // do something with the events
            // but it's kind of useless
            // it's still necessary to empty the event pool
        }

        let (width, height) = displayer.sdl_renderer().window().unwrap().size();
        mpv_gl.draw(0, width as i32, -(height as i32)).expect("Failed to draw ");
        displayer.display("0123456789ABCDEF0123456789abcdef0123456789ABCDEF");
        displayer.render();
    }
    info!("Exiting software peacefully");
}
