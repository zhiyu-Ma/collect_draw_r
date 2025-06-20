# 添加musl目标
rustup target add x86_64-unknown-linux-musl

# 安装musl工具链
sudo apt-get install musl-tools  # Debian/Ubuntu

# 编译
cargo build --release --target=x86_64-unknown-linux-musl