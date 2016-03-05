#![allow(dead_code)]

use std;
use std::ffi;
use std::ptr;
use std::result;

use libc;

use num::FromPrimitive;

use mpv_gen::*;

pub type Result<T> = result::Result<T, mpv_error>;

pub struct Mpv {
    handle: *mut mpv_handle,
}

impl Mpv {
    pub fn init() -> Result<Mpv> {
        let handle = unsafe { mpv_create() };
        if handle == ptr::null_mut() {
            return Err(Enum_mpv_error::MPV_ERROR_NOMEM);
        }

        let ret = unsafe { mpv_initialize(handle) };
        if ret < 0 {
            return Err(mpv_error::from_i32(ret).unwrap());
        }

        Ok(Mpv { handle: handle })
    }

    pub fn set_option(&self, name: &str, value: &str) {
        let name = ffi::CString::new(name).unwrap();
        let value = ffi::CString::new(value).unwrap();
        unsafe {
            assert!(mpv_set_option_string(self.handle, name.as_ptr(), value.as_ptr()) >= 0);
        }
    }

    pub fn get_opengl_context(&self,
                              get_proc_address: mpv_opengl_cb_get_proc_address_fn,
                              get_proc_address_ctx: *mut std::os::raw::c_void)
                              -> Result<OpenglContext> {
        OpenglContext::init(unsafe {
                                mpv_get_sub_api(self.handle, Enum_mpv_sub_api::MPV_SUB_API_OPENGL_CB) as *mut mpv_opengl_cb_context
                            },
                            get_proc_address,
                            get_proc_address_ctx)
    }

    pub fn command(&self, command: &[&str]) -> Result<()> {
        let command_cstring: Vec<_> = command.iter().map(|item| ffi::CString::new(*item).unwrap()).collect();
        let mut command_pointers: Vec<_> = command_cstring.iter().map(|item| item.as_ptr()).collect();
        command_pointers.push(ptr::null());

        let ret = unsafe{mpv_command(self.handle, command_pointers.as_mut_ptr())};
        if ret < 0 {
            Err(mpv_error::from_i32(ret).unwrap())
        } else {
            Ok(())
        }
    }
}

impl Drop for Mpv {
    fn drop(&mut self) {
        unsafe {
            mpv_terminate_destroy(self.handle);
        }
    }
}

pub struct OpenglContext {
    handle: *mut mpv_opengl_cb_context,
}

impl OpenglContext {
    fn init(ctx: *mut mpv_opengl_cb_context,
            get_proc_address: mpv_opengl_cb_get_proc_address_fn,
            get_proc_address_ctx: *mut std::os::raw::c_void)
            -> Result<OpenglContext> {
        assert!(!ctx.is_null());
        let ret = unsafe {
            mpv_opengl_cb_init_gl(ctx, ptr::null(), get_proc_address, get_proc_address_ctx)
        };
        if ret < 0 {
            Err(mpv_error::from_i32(ret).unwrap())
        } else {
            Ok(OpenglContext { handle: ctx })
        }
    }

    pub fn draw(&self, fbo: i32, width: i32, heigth: i32) {
        let ret = unsafe { mpv_opengl_cb_draw(self.handle, fbo, width, heigth) };
        assert!(ret >= 0);
    }

    //pub fn report_flip(&self, time: i64) -> Result<()> {
    //    let ret = unsafe { mpv_opengl_cb_report_flip(self.handle, time) };
    //    if ret < 0 {
    //        Err(mpv_error::from_i32(ret).unwrap())
    //    } else {
    //        Ok(())
    //    }
    //}
}

impl Drop for OpenglContext {
    fn drop(&mut self) {
        unsafe {
            mpv_opengl_cb_uninit_gl(self.handle);
        }
    }
}
