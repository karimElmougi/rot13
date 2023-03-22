use std::io::{Read, Write};
use std::os::unix::io::FromRawFd;

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let mut file = std::fs::File::open(filename).unwrap();

    const KB: usize = 1024;
    let mut buf = vec![0u8; 8 * KB];

    let mut stdout = unsafe { std::fs::File::from_raw_fd(1) };

    while let Ok(n) = file.read(&mut buf) {
        if n == 0 { break; }
        for c in buf[..n].iter_mut() {
            *c = rot13::branchless_sub_mul(*c as char) as u8;
        }
        stdout.write_all(&buf).unwrap();
    }
}
