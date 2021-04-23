# Compile Server Configuration

use CentOS-7-x86_64-Minimal-2009.iso
> http://mirrors.aliyun.com/centos/7/isos/x86_64/

**CentOS Compile toolchain**
```shell
yum install net-tools.x86_64
ifconfig

yum update
yum install install clang curl git openssl-devel
yum install clang
yum -y install gcc-c++
yum -y install gcc
```

**Manage the Rust toolchain**
```shell
curl https://sh.rustup.rs -sSf | sh
source ~/.cargo/env
```

**Configure the Rust toolchain**
```shell
rustup default stable
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly 
```
