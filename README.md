# Banksy Network

### Build the Banksy Node

```shell
cargo build --release
```

## Run

### Chain IDs

Golden Ratio
Phi
φ = 1.618

| Network Description       | Chain ID |
| ------------------------- | -------- |
| Local Parachain TestNet   | 1616     |
| Local Development TestNet | 1617     |
| Banksy Rococo             | 1618     |



 

### Single Node Development Chain

Purge any existing dev chain state:

```bash
./target/release/banksy-collator purge-chain --dev
```

Start a dev chain:

```bash
./target/release/banksy-collator --dev --ws-external --rpc-external --rpc-cors=all
```

### Rococo Chain
```shell
./target/release/banksy-collator --collator --tmp -- --chain rococo
```

```shell
./target/release/banksy-collator --ws-external --collator --tmp --parachain-id 2419 -- --execution wasm --chain ./resources/rococo.json
```

### Export genesis 

```shell
# Export genesis state
./target/release/banksy-collator export-genesis-state --parachain-id 2419 > genesis-state

# Export genesis wasm
./target/release/banksy-collator export-genesis-wasm > genesis-wasm
```




