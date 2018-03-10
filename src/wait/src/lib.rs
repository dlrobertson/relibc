//! sys/wait.h implementation for Redox, following
//! http://pubs.opengroup.org/onlinepubs/7908799/xsh/syswait.h.html

#![no_std]

extern crate platform;
extern crate resource;

use platform::types::*;
use resource::rusage;

#[no_mangle]
pub unsafe extern "C" fn wait(stat_loc: *mut c_int) -> pid_t {
    unimplemented!();
}

/*
 * TODO: implement rusage
 *
 * #[no_mangle]
 * Pub unsafe extern "C" fn wait3(stat_loc: *mut c_int, options: c_int, resource_usage: rusage) -> pid_t {
 *     unimplemented!();
 * }
 */

/*
 * TODO: implement idtype_t, id_t, and siginfo_t
 *
 * #[no_mangle]
 * pub unsafe extern "C" fn waitid(
 *     idtype: idtype_t,
 *     id: id_t,
 *     infop: siginfo_t,
 *     options: c_int
 *  ) -> c_int {
 *      unimplemented!();
 *  }
 */

#[no_mangle]
pub unsafe extern "C" fn waitpid(pid: pid_t, stat_loc: *mut c_int, options: c_int) -> pid_t {
    unimplemented!();
}
