use std::io::Read;
use std::time::Instant;

fn main() {
    let mut bytes_read = 0;
    let mut bytes_read_instant = 0;
    let start = Instant::now();
    let mut last_secs = 0;
    let mut input = std::io::stdin();
    let mut buffer: [u8; 1024] = [0; 1024];
    loop {
        match input.read(&mut buffer) {
            Ok(size) => {
                let _read = &buffer[0..size];
                bytes_read += size;
                bytes_read_instant += size;
                let elapsed_time = start.elapsed().as_secs();
                if elapsed_time != last_secs {
                    last_secs = elapsed_time;
                    // TODO: windowed averaging to avoid initial error
                    println!(
                        "Bytes/sec: {}, average: {}",
                        bytes_read_instant,
                        bytes_read / elapsed_time as usize
                    );

                    bytes_read_instant = 0;
                }
            }
            Err(_) => todo!(),
        }
    }
}
