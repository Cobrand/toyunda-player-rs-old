use std::ffi;
use std::ptr;
use std::result;

use libc;

use num::FromPrimitive;

enum MpvHandle {}
enum OpenglContextHandle {}
#[repr(C)]
#[derive(Debug)]
pub enum SubApi {
    Opengl = 1,
    // E0083
    Pote,
}
enum_from_primitive! {
#[repr(C)]
#[derive(Debug)]
pub enum Error {
    SUCCESS = 0,
    EVENT_QUEUE_FULL = -1,
    NOMEM = -2,
    UNINITIALIZED = -3,
    INVALID_PARAMETER = -4,
    OPTION_NOT_FOUND = -5,
    OPTION_FORMAT = -6,
    OPTION_ERROR = -7,
    PROPERTY_NOT_FOUND = -8,
    PROPERTY_FORMAT = -9,
    PROPERTY_UNAVAILABLE = -10,
    PROPERTY_ERROR = -11,
    COMMAND = -12,
    LOADING_FAILED = -13,
    AO_INIT_FAILED = -14,
    VO_INIT_FAILED = -15,
    NOTHING_TO_PLAY = -16,
    UNKNOWN_FORMAT = -17,
    UNSUPPORTED = -18,
    NOT_IMPLEMENTED = -19,
}
}
pub type Result<T> = result::Result<T, Error>;

pub type UpdateCallback = Fn(*const libc::c_void);
pub type GetProcAddressFn = Fn(*const libc::c_void, *const libc::c_char) -> *mut libc::c_void;

#[link(name = "mpv")]
extern "C" {
    fn mpv_free(data: *mut libc::c_void);
    fn mpv_create() -> *mut MpvHandle;
    fn mpv_initialize(handle: *mut MpvHandle) -> libc::c_int;
    fn mpv_terminate_destroy(mpv: *mut MpvHandle);
    fn mpv_set_option_string(ctx: *mut MpvHandle,
                             name: *const libc::c_char,
                             data: *const libc::c_char)
                             -> libc::c_int;
    fn mpv_get_sub_api(ctx: *mut MpvHandle, sub_api: SubApi) -> *mut OpenglContextHandle;

    fn mpv_opengl_cb_set_update_callback(ctx: *mut OpenglContextHandle);
    fn mpv_opengl_cb_init_gl(ctx: *mut OpenglContextHandle,
                             extensions: *const libc::c_char,
                             get_proc_address: *const GetProcAddressFn,
                             get_proc_address_ctx: *const libc::c_void)
                             -> libc::c_int;
    fn mpv_opengl_cb_draw(ctx: *mut OpenglContextHandle,
                          fbo: libc::c_int,
                          w: libc::c_int,
                          h: libc::c_int)
                          -> libc::c_int;
    fn mpv_opengl_cb_report_flip(ctx: *mut OpenglContextHandle,
                                 time: libc::int64_t)
                                 -> libc::c_int;
    fn mpv_opengl_cb_uninit_gl(ctx: *mut OpenglContextHandle) -> libc::c_int;
}

pub struct Mpv {
    handle: *mut MpvHandle,
}

impl Mpv {
    pub fn init() -> Result<Mpv> {
        let handle = unsafe{mpv_create()};
        if handle == ptr::null_mut() {
            return Err(Error::NOMEM);
        }

        let ret = unsafe { mpv_initialize(handle) };
        if ret < 0 {
            return Err(Error::from_i32(ret).unwrap());
        }

        Ok(Mpv { handle: handle })
    }

    pub fn set_option(&self, name: &ffi::CStr, value: &ffi::CStr) {
        unsafe {
            assert!(mpv_set_option_string(self.handle, name.as_ptr(), value.as_ptr()) >= 0);
        }
    }

    pub fn get_opengl_context(&self,
                              get_proc_address: *const GetProcAddressFn,
                              get_proc_address_ctx: *const libc::c_void)
                              -> Result<OpenglContext> {
        OpenglContext::init(unsafe { mpv_get_sub_api(self.handle, SubApi::Opengl) },
                            get_proc_address,
                            get_proc_address_ctx)
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
    handle: *mut OpenglContextHandle,
}

impl OpenglContext {
    fn init(ctx: *mut OpenglContextHandle,
            get_proc_address: *const GetProcAddressFn,
            get_proc_address_ctx: *const libc::c_void)
            -> Result<OpenglContext> {
        assert!(!ctx.is_null());
        let ret = unsafe {
            mpv_opengl_cb_init_gl(ctx, ptr::null(), get_proc_address, get_proc_address_ctx)
        };
        if ret < 0 {
            Err(Error::from_i32(ret).unwrap())
        } else {
            Ok(OpenglContext { handle: ctx })
        }
    }

    pub fn draw(&self, fbo: i32, width: i32, heigth: i32) {
        let ret = unsafe { mpv_opengl_cb_draw(self.handle, fbo, width, heigth) };
        assert!(ret >= 0);
    }

    pub fn report_flip(&self, time: i64) -> Result<()> {
        let ret = unsafe { mpv_opengl_cb_report_flip(self.handle, time) };
        if ret < 0 {
            Err(Error::from_i32(ret).unwrap())
        } else {
            Ok(())
        }
    }
}

impl Drop for OpenglContext {
    fn drop(&mut self) {
        unsafe {
            mpv_opengl_cb_uninit_gl(self.handle);
        }
    }
}
