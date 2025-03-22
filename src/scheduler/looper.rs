use crate::{
    activity::{ActivityUtils, get_tid_info::get_process_name},
    utils::sleep::sleep_secs,
};
use compact_str::CompactString;
use libc::pid_t;
use log::info;
// unsafe extern "C" {
// fn __llvm_profile_write_file() -> i32;
// }

pub struct Looper {
    pub activity_utils: ActivityUtils,
    pub global_package: CompactString,
    pub pid: pid_t,
}

impl Looper {
    pub fn new(activity_utils: ActivityUtils) -> Self {
        Self {
            activity_utils,
            global_package: CompactString::new(""),
            pid: -1,
        }
    }

    pub fn game_exit(&mut self) {
        info!("Exiting game\n");
        self.pid = -1;
    }

    pub fn enter_loop(&mut self) {
        'outer: loop {
            sleep_secs(1);
            {
                let pid = self.activity_utils.top_app_utils.get_top_pid();
                if self.pid == pid {
                    continue 'outer;
                }
                self.pid = pid;
                let name = get_process_name(pid).unwrap_or_default();
                self.global_package = name;
            }
            // unsafe {
            // __llvm_profile_write_file();
            // }
        }
    }
}
