use crate::{
    activity::{ActivityUtils, get_tid_info::get_process_name},
    config::PROFILE,
    utils::sleep::sleep_secs,
};
use compact_str::CompactString;
use libc::{kill, pid_t};
use likely_stable::unlikely;
use log::info;
// unsafe extern "C" {
// fn __llvm_profile_write_file() -> i32;
// }

pub struct Looper {
    pub activity_utils: ActivityUtils,
    pub global_package: CompactString,
    pub pid: pid_t,
    pub need_stop: bool,
}

impl Looper {
    pub fn new(activity_utils: ActivityUtils) -> Self {
        Self {
            activity_utils,
            global_package: CompactString::new(""),
            pid: -1,
            need_stop: false,
        }
    }

    fn wait_until_exit(&mut self) {
        loop {
            sleep_secs(1);
            let pid = self.activity_utils.top_app_utils.get_top_pid();
            if unlikely(pid != self.pid) {
                self.game_exit();
                return;
            }
        }
    }

    fn game_exit(&mut self) {
        if self.need_stop {
            info!("发送停止信号\n");
            let _ = unsafe { kill(self.pid, libc::SIGSTOP) };
            self.need_stop = false;
        }
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

            for i in &PROFILE.packages {
                if self.global_package == i {
                    info!("发送解冻信号\n");
                    let _ = unsafe { kill(self.pid, libc::SIGCONT) };
                    self.need_stop = true;
                    self.wait_until_exit();
                    continue 'outer;
                }
            }
            // unsafe {
            // __llvm_profile_write_file();
            // }
        }
    }
}
