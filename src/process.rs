use sysinfo::{System};
use sysinfo::Signal::Term;

pub fn find_and_terminate_process(command_fragment: &str) -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut terminated_count = 0;

    for (pid, process) in sys.processes() {
        let command_line = process.cmd()
            .iter()
            .map(|os_string| os_string.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ");

        if command_line.contains(command_fragment) {
            println!("🚨 Found target process: PID {} with command: {}", pid, command_line);

            if process.kill_with(Term).unwrap() {
                println!("✅ Successfully sent kill signal (SIGKILL) to process PID {}.", pid);
                terminated_count += 1;
            } else {
                eprintln!("❌ Failed to send kill signal to process PID {}.", pid);
            }
        }
    }
    
    terminated_count > 0
}