#![allow(dead_code)]

use std::error::Error;
use std::ffi;
use std::fmt;
use std::ptr;
use std::result;
use std::os::raw as libc;

use num::FromPrimitive;

use mpv_gen::*;

pub type Result<T> = result::Result<T, mpv_error>;

impl Error for Enum_mpv_error {
    fn description(&self) -> &str {
        let str_ptr = unsafe {mpv_error_string(*self as libc::c_int)};
        assert!(!str_ptr.is_null());
        unsafe { ffi::CStr::from_ptr(str_ptr).to_str().unwrap() }
    }
}

impl fmt::Display for Enum_mpv_error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({:?})", self.description(), self)
    }
}

pub struct Mpv {
    handle: *mut mpv_handle,
}

pub enum MpvFormat<'a> {
    RawMpvFormat {
        format: Enum_mpv_format,
        data: *mut libc::c_void,
    },
    Str(&'a str),
}

pub trait MpvFormatProperty : Clone {
    fn to_mpv_format(&mut self) -> MpvFormat;
}

impl MpvFormatProperty for f64 {
    fn to_mpv_format(&mut self) -> MpvFormat {
        let ptr = self as *mut _ as *mut libc::c_void;
        MpvFormat::RawMpvFormat {
            format: Enum_mpv_format::MPV_FORMAT_DOUBLE,
            data: ptr,
        }
    }
}

impl MpvFormatProperty for bool {
    fn to_mpv_format(&mut self) -> MpvFormat {
        let ptr = self as *mut _ as *mut libc::c_void;
        MpvFormat::RawMpvFormat {
            format: Enum_mpv_format::MPV_FORMAT_FLAG,
            data: ptr,
        }
    }
}

impl<'a> MpvFormatProperty for &'a str {
    fn to_mpv_format(&mut self) -> MpvFormat {
        MpvFormat::Str(self)
    }
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
                                mpv_get_sub_api(self.handle,
                                                Enum_mpv_sub_api::MPV_SUB_API_OPENGL_CB)
                            } as *mut mpv_opengl_cb_context,
                            get_proc_address,
                            get_proc_address_ctx)
    }

    pub fn command(&self, command: &[&str]) -> Result<()> {
        let command_cstring: Vec<_> = command.iter()
                                             .map(|item| ffi::CString::new(*item).unwrap())
                                             .collect();
        let mut command_pointers: Vec<_> = command_cstring.iter()
                                                          .map(|item| item.as_ptr())
                                                          .collect();
        command_pointers.push(ptr::null());

        let ret = unsafe { mpv_command(self.handle, command_pointers.as_mut_ptr()) };

        ret_to_result(ret, ())
    }

    pub fn wait_event(&self) -> Option<Struct_mpv_event> {
        let event = unsafe {
            let ptr = mpv_wait_event(self.handle, 0.0);
            if ptr.is_null() {
                panic!("Unexpected null ptr from mpv_wait_event");
            }
            *ptr
        };
        match event.event_id {
            Enum_mpv_event_id::MPV_EVENT_NONE => None,
            _ => Some(event),
        }
    }

    pub fn set_property<T: MpvFormatProperty>(&self, property: &str, mut value: T) -> Result<()> {
        let format_struct: MpvFormat = value.to_mpv_format();
        let ret = match format_struct {
            MpvFormat::RawMpvFormat { format, data: ptr } => unsafe {
                mpv_set_property(self.handle,
                                 ffi::CString::new(property).unwrap().as_ptr(),
                                 format,
                                 ptr)
            },
            MpvFormat::Str(string) => unsafe {
                mpv_set_property_string(self.handle,
                                        ffi::CString::new(property).unwrap().as_ptr(),
                                        ffi::CString::new(string).unwrap().as_ptr())
            },
        };
        ret_to_result(ret, ())
    }

    pub fn get_property_string(&self, property: &str) -> Option<String> {
        let ret = unsafe {
            mpv_get_property_string(self.handle,
                                    ffi::CString::new(property)
                                        .unwrap()
                                        .as_ptr())
        };
        if ret.is_null() {
            return None;
        }

        let ret_string = unsafe { ffi::CString::from_raw(ret) }
                             .into_string()
                             .unwrap();
        unsafe { mpv_free(ret as *mut libc::c_void) };
        Some(ret_string)
    }

    pub fn set_option<T: MpvFormatProperty>(&self, option: &str, mut value: T) -> Result<()> {
        let format_struct: MpvFormat = value.to_mpv_format();
        let ret = match format_struct {
            MpvFormat::RawMpvFormat { format, data: ptr } => unsafe {
                mpv_set_option(self.handle,
                               ffi::CString::new(option).unwrap().as_ptr(),
                               format,
                               ptr)
            },
            MpvFormat::Str(string) => unsafe {
                mpv_set_option_string(self.handle,
                                      ffi::CString::new(option).unwrap().as_ptr(),
                                      ffi::CString::new(string).unwrap().as_ptr())
            },
        };
        ret_to_result(ret, ())
    }

    pub fn debug(&self) {
        unsafe {
            let msg = ffi::CString::new("trace").unwrap();
            mpv_request_log_messages(self.handle, msg.as_ptr());
        }
    }
}

fn ret_to_result<T>(ret: i32, default: T) -> Result<T> {
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

    pub fn draw(&self, fbo: i32, width: i32, heigth: i32) -> Result<()> {
        let ret = unsafe { mpv_opengl_cb_draw(self.handle, fbo, width, heigth) };
        ret_to_result(ret, ())
    }

    // pub fn report_flip(&self, time: i64) -> Result<()> {
    //    let ret = unsafe { mpv_opengl_cb_report_flip(self.handle, time) };
    //    ret_to_result(ret, ())
    // }
}

impl Drop for OpenglContext {
    fn drop(&mut self) {
        unsafe {
            mpv_opengl_cb_uninit_gl(self.handle);
        }
    }
}
