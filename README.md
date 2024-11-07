# acala-node

This is the client node for Acala and Karura. For runtime code, please refer to [Acala repo](https://github.com/AcalaNetwork/Acala).

# Building

```bash
curl https://sh.rustup.rs -sSf | sh
```

You may need additional dependencies, checkout [substrate.io](https://docs.substrate.io/v3/getting-started/installation) for more info

```bash
sudo apt-get install -y git clang curl make libssl-dev llvm libudev-dev protobuf-compiler
```

Debug build:

```bash
cargo build
```

Release build:

```bash
cargo build --release
```
