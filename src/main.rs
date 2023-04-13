use std::collections::VecDeque;
use std::io::Read;
use std::time::Instant;

fn main() {
    let mut bytes_read_instant = 0;
    let start = Instant::now();
    let mut last_secs = 0;
    let mut input = std::io::stdin();
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut values = VecDeque::<u64>::with_capacity(10);
    loop {
        match input.read(&mut buffer) {
            Ok(size) => {
                bytes_read_instant += size;
                let elapsed_time = start.elapsed().as_secs();
                if elapsed_time != last_secs {
                    last_secs = elapsed_time;
                    values.push_front(bytes_read_instant as u64);

                    if values.len() > 10 {
                        values.pop_back();
                    }
                    let average: u64 = values.iter().sum::<u64>() / values.len() as u64;
                    println!("Bytes/sec: {}, average: {}", bytes_read_instant, average);
                    bytes_read_instant = 0;
                }
            }
            Err(_) => todo!(),
        }
    }
}
