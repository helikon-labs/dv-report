#!/usr/bin/env bash
set -e

# polkadot
echo "Get Polkadot metadata."
subxt metadata --url "wss://rpc.helikon.io/asset-hub-polkadot" -o ./polkadot.scale
echo "Generate Polkadot runtime code from metadata."
subxt codegen --file ./polkadot.scale --no-docs \
  --substitute-type sp_arithmetic::per_things::PerU16=::core::primitive::u16 \
  --substitute-type sp_arithmetic::per_things::Perbill=::core::primitive::u32 \
   --derive-for-type asset_hub_polkadot_runtime::RuntimeCall=Clone,recursive \
  --derive-for-type asset_hub_polkadot_runtime::RuntimeCall=::subxt::ext::codec::Encode,recursive \
  --derive-for-type asset_hub_polkadot_runtime::RuntimeCall=::subxt::ext::codec::Decode,recursive \
  | rustfmt --edition=2021 --emit=stdout > ../dv-report-metadata/src/runtime/polkadot.rs
echo "Remove Polkadot metadata."
rm ./polkadot.scale

# polkadot current
echo "Get Polkadot current metadata."
subxt metadata --url "wss://rpc.helikon.io/asset-hub-polkadot" --pallets Referenda,ConvictionVoting,Utility,Multisig,Proxy -o ./polkadot.scale
echo "Generate Polkadot current runtime code from metadata."
subxt codegen --file ./polkadot.scale --no-docs  \
  --substitute-type sp_arithmetic::per_things::PerU16=::core::primitive::u16 \
  --substitute-type sp_arithmetic::per_things::Perbill=::core::primitive::u32 \
  --derive-for-type asset_hub_polkadot_runtime::RuntimeCall=Clone,recursive \
  --derive-for-type asset_hub_polkadot_runtime::RuntimeCall=::subxt::ext::codec::Encode,recursive \
  --derive-for-type asset_hub_polkadot_runtime::RuntimeCall=::subxt::ext::codec::Decode,recursive \
  | rustfmt --edition=2021 --emit=stdout > ../dv-report-metadata/src/runtime/polkadot_current.rs
echo "Remove Polkadot current metadata."
rm ./polkadot.scale

# kusama
echo "Get Kusama metadata."
subxt metadata --url "wss://rpc.helikon.io/asset-hub-kusama" -o ./kusama.scale
echo "Generate Kusama runtime code from metadata."
subxt codegen --file ./kusama.scale --no-docs \
  --substitute-type sp_arithmetic::per_things::PerU16=::core::primitive::u16 \
  --substitute-type sp_arithmetic::per_things::Perbill=::core::primitive::u32 \
  --derive-for-type asset_hub_kusama_runtime::RuntimeCall=Clone,recursive \
  --derive-for-type asset_hub_kusama_runtime::RuntimeCall=::subxt::ext::codec::Encode,recursive \
  --derive-for-type asset_hub_kusama_runtime::RuntimeCall=::subxt::ext::codec::Decode,recursive \
  | rustfmt --edition=2021 --emit=stdout > ../dv-report-metadata/src/runtime/kusama.rs
echo "Remove Kusama metadata."
rm ./kusama.scale

# kusama current
echo "Get Kusama current metadata."
subxt metadata --url "wss://rpc.helikon.io/asset-hub-kusama" --pallets Referenda,ConvictionVoting,Utility,Multisig,Proxy -o ./kusama.scale
echo "Generate Kusama current runtime code from metadata."
subxt codegen --file ./kusama.scale --no-docs  \
  --substitute-type sp_arithmetic::per_things::PerU16=::core::primitive::u16 \
  --substitute-type sp_arithmetic::per_things::Perbill=::core::primitive::u32 \
  --derive-for-type asset_hub_kusama_runtime::RuntimeCall=Clone,recursive \
  --derive-for-type asset_hub_kusama_runtime::RuntimeCall=::subxt::ext::codec::Encode,recursive \
  --derive-for-type asset_hub_kusama_runtime::RuntimeCall=::subxt::ext::codec::Decode,recursive \
  | rustfmt --edition=2021 --emit=stdout > ../dv-report-metadata/src/runtime/kusama_current.rs
echo "Remove Kusama current metadata."
rm ./kusama.scale