# zsocks

## Usage

as env vars
```bash
LISTEN_ADDR=127.0.0.1:9800 USERNAME=admin PASSWORD=nimda ./target/debug/zsocks
```
or 
```bash
./target/debug/zsocks --listen-addr=127.0.0.1:9800 --password=admin --username=nimda
```

more logging
```bash
RUST_LOG=info LISTEN_ADDR=127.0.0.1:9800 USERNAME=admin PASSWORD=nimda ./target/debug/zsocks
```