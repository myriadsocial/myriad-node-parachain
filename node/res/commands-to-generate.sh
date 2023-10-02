./target/release/myriad build-spec --disable-default-bootnode > plain-parachain-chainspec.json
./target/release/myriad build-spec --chain edited-parachain-chainspec.json --disable-default-bootnode --raw > raw-parachain-chainspec.json
./target/release/myriad export-genesis-wasm --chain raw-parachain-chainspec.json para-4242-wasm
./target/release/myriad export-genesis-state --chain raw-parachain-chainspec.json para-4242-genesis-state