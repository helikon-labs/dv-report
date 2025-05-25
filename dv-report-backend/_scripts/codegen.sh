#!/usr/bin/env bash
set -e
# polkadot
subxt metadata --url "wss://rpc.helikon.io/polkadot" --pallets Referenda,ConvictionVoting,Utility,Multisig,Proxy -o ./polkadot.scale
subxt codegen --file ./polkadot.scale --no-docs | rustfmt --edition=2021 --emit=stdout > ../dv-report-types/src/metadata/polkadot.rs
rm ./polkadot.scale

# kusama
subxt metadata --url "wss://rpc.helikon.io/kusama" --pallets Referenda,ConvictionVoting,Utility,Multisig,Proxy -o ./kusama.scale
subxt codegen --file ./kusama.scale --no-docs | rustfmt --edition=2021 --emit=stdout > ../dv-report-types/src/metadata/kusama.rs
rm ./kusama.scale