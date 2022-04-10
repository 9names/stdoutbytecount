A simple program I wrote for getting a simple representative measure of RTT performance.

Example usage (from an instrumented version of the stm32h7 blinky program, using probe-run)  
```console
cargo run -r --example blinky --features stm32h743v,revision_v,rt,log-rtt -- --speed 20000 |stdoutbytecount
```