extern crate rustc_serialize;
extern crate docopt;

#[macro_use]
extern crate enum_primitive;
extern crate num;

extern crate gl;
#[macro_use]
extern crate glium;

#[macro_use]
extern crate log;
extern crate env_logger;

use glium::{DisplayBuild,Surface};

use std::ffi::CStr;
use std::os::raw as libc;
use std::ops::Deref;

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
    let arg: &glium::backend::glutin_backend::WinRef = &*(arg as *mut glium::backend::glutin_backend::WinRef);
    let name = CStr::from_ptr(name).to_str().unwrap();
    arg.get_proc_address(name) as *mut libc::c_void
}

fn get_mpv_gl(mpv: &mpv::Mpv, winref: &mut glium::backend::glutin_backend::WinRef) -> mpv::OpenglContext {
    let ptr = winref as *mut _ as *mut libc::c_void;
    mpv.get_opengl_context(Some(do_pote), ptr).unwrap()
}

fn main() {
    env_logger::init().unwrap();

    let args: CmdArgs = docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title(format!("Hello world"))
        .build_glium()
        .unwrap();

    let mpv = mpv::Mpv::init().unwrap();
    let mpv_gl = get_mpv_gl(&mpv, &mut display.get_window().unwrap());

    mpv.set_option("vo", "opengl-cb").unwrap();
    mpv.set_option("sid", "no").unwrap();
    mpv.command(&["loadfile", &args.arg_file as &str]).unwrap();

    //////////////

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    ///////////:

    'running: loop {
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => break 'running,   // the window has been closed by the user
                _ => {}
            }
        };
        let (width, height) = display.get_window().unwrap().get_inner_size_pixels().unwrap();
        mpv_gl.draw(0, width as i32, -(height as i32)).unwrap();
        let mut target = display.draw();
        // for event in event_pump.poll_iter() {
        //     match event {
        //         Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
        //             break 'running
        //         },
        //         Event::KeyDown { keycode: Some(Keycode::Space),repeat: false, .. } => {
        //             match mpv.get_property_string("pause") {
        //                 "yes" => {mpv.set_property("pause","no").unwrap();},
        //                 "no" => {mpv.set_property("pause","yes").unwrap();},
        //                 _ => {panic!("unexpected answer from get_property_string");}
        //             }
        //         },
        //         Event::KeyDown { keycode: Some(Keycode::Kp9), repeat: false, .. } => {mpv.set_property("speed",0.9).unwrap();},
        //         Event::KeyDown { keycode: Some(Keycode::Kp8), repeat: false, .. } => {mpv.set_property("speed",0.8).unwrap();},
        //         Event::KeyDown { keycode: Some(Keycode::Kp7), repeat: false, .. } => {mpv.set_property("speed",0.7).unwrap();},
        //         Event::KeyDown { keycode: Some(Keycode::Kp6), repeat: false, .. } => {mpv.set_property("speed",0.6).unwrap();},
        //         Event::KeyDown { keycode: Some(Keycode::Kp5), repeat: false, .. } => {mpv.set_property("speed",0.5).unwrap();},
        //         Event::KeyDown { keycode: Some(Keycode::Kp4), repeat: false, .. } => {mpv.set_property("speed",0.4).unwrap();},
        //         Event::KeyDown { keycode: Some(Keycode::Kp3), repeat: false, .. } => {mpv.set_property("speed",0.3).unwrap();},
        //         Event::KeyDown { keycode: Some(Keycode::Kp2), repeat: false, .. } => {mpv.set_property("speed",0.2).unwrap();},
        //         Event::KeyDown { keycode: Some(Keycode::Kp1), repeat: false, .. } => {mpv.set_property("speed",0.1).unwrap();},
        //         Event::KeyDown { keycode: Some(Keycode::Kp0), repeat: false, .. } => {mpv.set_property("speed",1.0).unwrap();},
        //         Event::KeyDown { keycode: Some(Keycode::F), repeat: false, .. } => {
        //             if (renderer.window().unwrap().window_flags() &
        //                 (SDL_WindowFlags::SDL_WINDOW_FULLSCREEN as u32)) != 0 {
        //                 renderer.window_mut().unwrap().set_fullscreen(FullscreenType::Off)
        //             } else {
        //                 renderer.window_mut().unwrap().set_fullscreen(FullscreenType::Desktop)
        //             }
        //             .unwrap();
        //         }
        //         _ => {}
        //     }
        // }
        while let Some(_) = mpv.wait_event() {
            // do something with the events
            // but it's kind of useless
            // it's still necessary to empty the event pool
        }


        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
    }
}
