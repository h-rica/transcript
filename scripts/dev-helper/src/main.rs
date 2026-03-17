use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::{self, Command, ExitCode, Stdio},
    thread,
    time::Duration,
};

use sysinfo::{Pid, ProcessesToUpdate, Signal, System};

const STATE_FILE: &str = ".dev-helper-state";

#[derive(Clone, Debug)]
enum Mode {
    Start { repo_root: PathBuf },
    Cleanup { repo_root: PathBuf, app_pid: Option<u32> },
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<ExitCode, Box<dyn Error>> {
    let mode = parse_args(env::args().skip(1))?;

    match mode {
        Mode::Start { repo_root } => start(repo_root),
        Mode::Cleanup { repo_root, app_pid } => {
            cleanup(&repo_root, app_pid)?;
            Ok(ExitCode::SUCCESS)
        }
    }
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Mode, Box<dyn Error>> {
    let Some(command) = args.next() else {
        return Err("missing command: expected `start` or `cleanup`".into());
    };

    let mut repo_root = None;
    let mut app_pid = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--repo-root" => {
                let value = args.next().ok_or("missing value for --repo-root")?;
                repo_root = Some(PathBuf::from(value));
            }
            "--app-pid" => {
                let value = args.next().ok_or("missing value for --app-pid")?;
                app_pid = Some(value.parse::<u32>()?);
            }
            other => return Err(format!("unexpected argument: {other}").into()),
        }
    }

    let repo_root = match repo_root {
        Some(path) => path,
        None => env::current_dir()?,
    };

    match command.as_str() {
        "start" => Ok(Mode::Start { repo_root }),
        "cleanup" => Ok(Mode::Cleanup { repo_root, app_pid }),
        other => Err(format!("unknown command: {other}").into()),
    }
}

fn start(repo_root: PathBuf) -> Result<ExitCode, Box<dyn Error>> {
    cleanup(&repo_root, None)?;

    let mut trunk = Command::new("trunk")
        .arg("serve")
        .current_dir(&repo_root)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    write_state(&repo_root, trunk.id())?;
    let status = trunk.wait()?;
    clear_state(&repo_root);

    Ok(ExitCode::from(status.code().unwrap_or(1) as u8))
}

fn cleanup(repo_root: &Path, app_pid: Option<u32>) -> Result<(), Box<dyn Error>> {
    let repo_root = fs::canonicalize(repo_root).unwrap_or_else(|_| repo_root.to_path_buf());
    let mut system = System::new_all();
    system.refresh_processes(ProcessesToUpdate::All, true);

    let current_pid = Pid::from_u32(process::id());
    let mut to_kill = Vec::new();

    if let Some(trunk_pid) = read_state(&repo_root) {
        to_kill.push(Pid::from_u32(trunk_pid));
        to_kill.extend(ancestor_pids(&system, Pid::from_u32(trunk_pid), 6));
    }

    if let Some(app_pid) = app_pid {
        let app_pid = Pid::from_u32(app_pid);
        to_kill.extend(ancestor_pids(&system, app_pid, 6));
    }

    let repo_app_root = repo_root.join("src-tauri");
    for (pid, process) in system.processes() {
        if *pid == current_pid {
            continue;
        }

        if process.name().to_string_lossy() == "app" {
            if let Some(exe) = process.exe() {
                if exe.starts_with(&repo_app_root) {
                    to_kill.push(*pid);
                    to_kill.extend(ancestor_pids(&system, *pid, 6));
                }
            }
        }
    }

    to_kill.sort_unstable_by_key(|pid| pid.as_u32());
    to_kill.dedup();

    for pid in to_kill {
        if pid == current_pid {
            continue;
        }

        if let Some(process) = system.process(pid) {
            let killed = process.kill_with(Signal::Kill).unwrap_or_else(|| process.kill());
            if !killed {
                let _ = process.kill();
            }
        }
    }

    thread::sleep(Duration::from_millis(300));

    remove_dir_if_exists(&repo_root.join("dist"));
    remove_dir_if_exists(&repo_root.join(".trunk"));
    clear_state(&repo_root);

    Ok(())
}

fn ancestor_pids(system: &System, start: Pid, max_depth: usize) -> Vec<Pid> {
    let mut current = start;
    let mut ancestors = Vec::new();

    for _ in 0..max_depth {
        let Some(process) = system.process(current) else {
            break;
        };

        let Some(parent) = process.parent() else {
            break;
        };

        ancestors.push(parent);
        current = parent;
    }

    ancestors
}

fn state_path(repo_root: &Path) -> PathBuf {
    repo_root.join(STATE_FILE)
}

fn write_state(repo_root: &Path, trunk_pid: u32) -> Result<(), Box<dyn Error>> {
    fs::write(state_path(repo_root), trunk_pid.to_string())?;
    Ok(())
}

fn read_state(repo_root: &Path) -> Option<u32> {
    fs::read_to_string(state_path(repo_root))
        .ok()?
        .trim()
        .parse()
        .ok()
}

fn clear_state(repo_root: &Path) {
    let _ = fs::remove_file(state_path(repo_root));
}

fn remove_dir_if_exists(path: &Path) {
    if path.exists() {
        let _ = fs::remove_dir_all(path);
    }
}
