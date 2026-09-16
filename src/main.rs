use nix::unistd::read;
use nix::unistd::write;
use nix::unistd::access;
use nix::unistd::{fork, ForkResult};
use nix::unistd::execve;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::AccessFlags;
use std::os::fd::BorrowedFd;
use std::path::Path;
use std::ffi::CString;

const BUFFER_SIZE: usize = 64;

fn resolve_path(cmd: &str) -> Option<String> {
    if cmd.contains('/') {
        if access(Path::new(cmd), AccessFlags::X_OK).is_ok() {
            return Some(cmd.to_string());
        }
        return None
    }
    let path_var = std::env::var("PATH").unwrap_or_default();
    let paths: Vec<&str> = path_var
        .split(':')
        .filter(|s| !s.is_empty())
        .collect();
    for dir in paths {
        let full_path = format!("{dir}/{cmd}");
        if access(Path::new(&full_path), AccessFlags::X_OK).is_ok() {
            return Some(full_path)
        }
    }
    None
}

fn  line_editor() -> Option<String> {
    let stderr = unsafe { BorrowedFd::borrow_raw(2) };

    let mut buffer = [0u8; BUFFER_SIZE];
    let mut size = 0;

    let _ = write(stderr, b"\x1b[31;1m > \x1b[0m");
    while size < BUFFER_SIZE {
        let mut c = [0u8; 1];

        match read(0, &mut c) {
            Ok(0) => return None,
            Err(_) => return None,
            Ok(_) => {}
        }

        if c[0] == b'\n' {
            break ;
        }
        buffer[size] = c[0];
        size += 1;
    }
    std::str::from_utf8(&buffer[..size])
        .ok()
        .map(|s| s.to_string())
}

fn execute_command(resolved_path: &str, args: &[&str]) {
    let c_path = CString::new(resolved_path).expect("CString conversion failed");
    let c_args: Vec<CString> = args
        .iter()
        .map(|&arg| CString::new(arg).expect("CString conversion failed"))
        .collect();
    let c_env: Vec<CString> = std::env::vars()
        .map(|(k, v)| CString::new(format!("{k}={v}")).unwrap())
        .collect();

    match unsafe { fork() } {
        Err(err) => {
            eprintln!("fork failed: {err}");
        }
        Ok(ForkResult::Child) => {
            let _ = execve(&c_path, &c_args, &c_env);
            eprintln!("execve failed");
            std::process::exit(127);
        }
        Ok(ForkResult::Parent {child}) => {
            match waitpid(child, None) {
                Ok(WaitStatus::Exited(_pid, _status)) => {}
                Ok(WaitStatus::Signaled(_pid, signal, _core_dump)) => {
                    eprintln!("Killed by signal: {:?}", signal);
                }
                _ => {}
            }
        }
    }
}

fn  main() {
    loop {
        let line = match line_editor() {
            Some(l) => l,
            None => std::process::exit(1),
        };
        let tokens: Vec<&str> = line
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect();
        if tokens.is_empty() {
            continue;
        }
        if tokens[0] == "exit" {
            break ;
        }
        match resolve_path(tokens[0]) {
            Some(path) => {
                execute_command(&path, &tokens);
            }
            None => {
                eprintln!("{}: command not found", tokens[0]);
            }
        }
    }
}
