#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use libc::{tm, clock_t, time_t};
unsafe extern "C" {
    fn __errno_location() -> *mut i32;
    fn setlocale(__category: i32, __locale: *const libc::c_char) -> *mut libc::c_char;
    fn exit(_: i32) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn mkstemp(__template: *mut libc::c_char) -> i32;
    fn system(__command: *const libc::c_char) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> i32;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> i32;
    fn clock() -> clock_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn difftime(__time1: time_t, __time0: time_t) -> f64;
    fn mktime(__tp: *mut tm) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: u64,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> u64;
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn close(__fd: i32) -> i32;
}
