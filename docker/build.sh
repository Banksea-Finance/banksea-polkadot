#!/usr/bin/env bash

cp ../../target/release/banksy-collator bin/banksy-collator
docker build -t banksyfinance/banksy:devnet .