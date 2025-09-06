use std::collections::HashSet;
use sysinfo::{Pid, Process, System};
use serde_json;
use serde::{Deserialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
struct FlaggedList {
    list: Vec<String>,
}

fn deserialize_from_file() -> Result<FlaggedList, Box<dyn Error>> {
    let path = format!("{}/flaggedList.json", env!("CARGO_MANIFEST_DIR"));
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let name_list: FlaggedList = serde_json::from_reader(reader)?;
    Ok(name_list)
}
pub fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    collect_processes_list(&sys);

    match deserialize_from_file() {
        Ok(list) => {
            println!("Loaded list: {:?}", list);
            kill_processes(&list, &sys);
        },
        Err(e) => println!("Error: {}", e),
    }

}

fn kill_processes(list: &FlaggedList, sys: &System) {
    for process_name in &list.list {
        let mut found = false;
        for process in sys.processes().values() {
            if process.name().to_string_lossy().as_ref() == process_name {
                found = true;
                if process.kill() {
                    println!("Killed process {}", process_name);
                } else {
                    println!("Failed to kill process {}", process_name);
                }
            }
        }
        if !found {
            println!("Process not found: {}", process_name);
        }
    }
}

fn collect_processes_list(sys: &System) -> HashSet<(&Pid, String)> {
    let mut seen = HashSet::new();

    for (pid, process) in sys.processes() {
        if let Some(path) = process.exe() {
            if let Some(parent_pid) = process.parent() {
                if let Some(parent_proc) = sys.process(parent_pid) {
                    if parent_proc.exe() == process.exe() {
                        continue;
                    }
                }
            }

            let exe_str = path.display().to_string();

            // Insert the tuple (pid, exe_str) into the HashSet
            if seen.insert((pid, exe_str.clone())) {
                println!("[{pid}] {} -> {}", process.name().to_string_lossy(), exe_str);
            }
        }
    }

    seen
}
