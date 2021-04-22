# Banksy Network

### Build the Banksy Node

```shell
cargo build --release
```

## Run

### Chain IDs

| Network Description       | Chain ID |
| ------------------------- | -------- |
| Local Parachain TestNet   | 1024     |
| Local Development TestNet | 1024     |
| Banksy Rococo             | 1024     |



 

### Single Node Development Chain

Purge any existing dev chain state:

```bash
./target/release/banksy-collator purge-chain --dev
```

Start a dev chain:

```bash
./target/release/banksy-collator --dev --ws-external --rpc-external --rpc-cors=all
```





