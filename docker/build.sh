#!/usr/bin/env bash

cp ../../target/release/banksy-collator bin/banksy-collator
docker build -t banksy/banksy:devnet .