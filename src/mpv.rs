use std::ffi;
use std::ptr;
use std::result;

use libc;

use num::FromPrimitive;

enum MpvHandle {}
enum OpenglContextHandle {}
#[repr(C)]
pub enum SubApi {
    Opengl = 1,
    // E0083
    Pote,
}
enum_from_primitive! {
#[repr(C)]
pub enum Error {
    MPV_ERROR_SUCCESS = 0,
    MPV_ERROR_EVENT_QUEUE_FULL = -1,
    MPV_ERROR_NOMEM = -2,
    MPV_ERROR_UNINITIALIZED = -3,
    MPV_ERROR_INVALID_PARAMETER = -4,
    MPV_ERROR_OPTION_NOT_FOUND = -5,
    MPV_ERROR_OPTION_FORMAT = -6,
    MPV_ERROR_OPTION_ERROR = -7,
    MPV_ERROR_PROPERTY_NOT_FOUND = -8,
    MPV_ERROR_PROPERTY_FORMAT = -9,
    MPV_ERROR_PROPERTY_UNAVAILABLE = -10,
    MPV_ERROR_PROPERTY_ERROR = -11,
    MPV_ERROR_COMMAND = -12,
    MPV_ERROR_LOADING_FAILED = -13,
    MPV_ERROR_AO_INIT_FAILED = -14,
    MPV_ERROR_VO_INIT_FAILED = -15,
    MPV_ERROR_NOTHING_TO_PLAY = -16,
    MPV_ERROR_UNKNOWN_FORMAT = -17,
    MPV_ERROR_UNSUPPORTED = -18,
    MPV_ERROR_NOT_IMPLEMENTED = -19,
}
}
pub type Result<T> = result::Result<T, Error>;

pub type UpdateCallback = Fn(*const libc::c_void);
pub type GetProcAddressFn = Fn(*const libc::c_void, *const libc::c_char) -> *mut libc::c_void;

#[link(name = "mpv")]
extern "C" {
    fn mpv_free(data: *mut libc::c_void);
    fn mpv_create() -> *mut MpvHandle;
    fn mpv_initialize(handle: *mut MpvHandle);
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
    pub fn init() -> Mpv {
        Mpv {
            handle: unsafe {
                let handle = mpv_create();
                mpv_initialize(handle);
                handle
            },
        }
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
            Err(Error::from_i32(ret).unwrap_or(Error::MPV_ERROR_SUCCESS))
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
