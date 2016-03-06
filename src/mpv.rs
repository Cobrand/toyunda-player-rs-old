#![allow(dead_code)]

use std;
use std::ffi;
use std::ptr;
use std::result;
use std::mem;
use std::option::Option;
use std::os::raw as libc;

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

        ret_to_result(ret, Mpv { handle: handle })
    }

    pub fn get_opengl_context(&self,
                              get_proc_address: mpv_opengl_cb_get_proc_address_fn,
                              get_proc_address_ctx: *mut libc::c_void)
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

        ret_to_result(())
    }

    pub fn wait_event(&self) -> Option<Struct_mpv_event> {
        unsafe {
            let ret = *mpv_wait_event(self.handle,0.0);
            match ret.event_id {
                Enum_mpv_event_id::MPV_EVENT_NONE => None,
                _ => Some(ret)
            }
        }
    }

    pub fn set_property_float(&self, property: &str, mut value: f64) -> Result<()> {
        let ptr = &mut value as *mut _ as *mut libc::c_void;
        let ret = unsafe {
            mpv_set_property(self.handle,
                            ffi::CString::new(property).unwrap().as_ptr(),
                            Enum_mpv_format::MPV_FORMAT_DOUBLE,
                            ptr)
        } ;

        ret_to_result(())
    }

    pub fn set_property_string(&self,property:&str,value:&str) -> Result<()> {
        let ret = unsafe {
            mpv_set_property_string(self.handle,
                                    ffi::CString::new(property).unwrap().as_ptr(),
                                    ffi::CString::new(value).unwrap().as_ptr())
        } ;

        ret_to_result(())
    }

    pub fn get_property_string(&self,property:&str) -> &str {
        unsafe {
            ffi::CStr::from_ptr(
                mpv_get_property_string(self.handle,ffi::CString::new(property).unwrap().as_ptr())
            ).to_str().unwrap()
        }
    }

    pub fn set_option_string(&self,property:&str,value:&str) -> Result<()> {
        let ret = unsafe {
            mpv_set_option_string(self.handle,
                                    ffi::CString::new(property).unwrap().as_ptr(),
                                    ffi::CString::new(value).unwrap().as_ptr())
        } ;

        ret_to_result(ret, ())
    }
}

fn ret_to_result<T>(ret: i32, default: T) -> Error<T> {
    if ret < 0 {
        Err(mpv_error::from_i32(ret).unwrap())
    } else {
        Ok(default)
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
            get_proc_address_ctx: *mut libc::c_void)
            -> Result<OpenglContext> {
        assert!(!ctx.is_null());
        let ret = unsafe {
            mpv_opengl_cb_init_gl(ctx, ptr::null(), get_proc_address, get_proc_address_ctx)
        };

        ret_to_result(ret, OpenglContext { handle: ctx })
    }

    pub fn draw(&self, fbo: i32, width: i32, heigth: i32) {
        let ret = unsafe { mpv_opengl_cb_draw(self.handle, fbo, width, heigth) };
        assert!(ret >= 0);
    }

    //pub fn report_flip(&self, time: i64) -> Result<()> {
    //    let ret = unsafe { mpv_opengl_cb_report_flip(self.handle, time) };
    //    ret_to_result(ret, ())
    //}
}

impl Drop for OpenglContext {
    fn drop(&mut self) {
        unsafe {
            mpv_opengl_cb_uninit_gl(self.handle);
        }
    }
}
