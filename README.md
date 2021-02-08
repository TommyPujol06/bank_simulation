# Bank simulator

## How to run

*You will need at least 1 GPU that supports cuda and have cuda installed*

```bash
rustup target add nvptx64-nvidia-cuda
export LIBRARY_PATH=/opt/cuda/lib64/
cargo install ptx-linker
cargo build --release
cargo run --release
```

