# Shadows Network

### Build the Banksy Node

```shell
cargo build --release
```

## Run

### Single Node Development Chain

### Single Node Development Chain

Purge any existing dev chain state:

```bash
./target/release/banksy-collator purge-chain --dev
```

Start a dev chain:

```bash
./target/release/banksy-collator --dev --ws-external --rpc-external --rpc-cors=all
```

 



