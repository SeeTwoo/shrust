use nix::unistd::read;
use nix::unistd::write;
use std::os::fd::BorrowedFd;

fn  line_editor() -> Option<String> {
    let stdin = unsafe { BorrowedFd::borrow_raw(0) };
    let stderr = unsafe { BorrowedFd::borrow_raw(2) };

    write(stderr, b" > ");
    loop {
        let mut byte = [0u8, 1];

        match read(stdin, &mut byte) {
            Ok(0) => {
                break
            }
            Ok(n) => {

            }
            Err(e) => {

            }
        }
    }
}

fn  main() {

    loop {
        let line = line_editor();
        if line == "exit" {
            break
        }

    }
}
