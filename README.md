# treat-bot

## Setup

- Install Rust
- Install cross
- Add target for Raspberry Pi

```bash
rustup target add aarch64-unknown-linux-gnu
```

## Build

- Building can be most easily done using cross

```bash
cross build --target aarch64-unknown-linux-gnu --release
```

Output will be in `target` folder

## Deploy

SCP is simplest for the time being

```bash
scp target/aarch64-unknown-linux-gnu/release/treat-bot user@ip:~/
ssh pi@pi-robot.local "~/robot-app"
```

```bash

scp target/aarch64-unknown-linux-gnu/release/treat-bot john@10.1.38.218:~/
```

### Run

- SSH in to pi
- CHMOD the binary

```bash
 chmod +x your_project_name
```

Execute

```bash
./your_project_name
```

### Testing

To run clippy you can use

```bash
cross clippy --target aarch64-unknown-linux-gnu
```
