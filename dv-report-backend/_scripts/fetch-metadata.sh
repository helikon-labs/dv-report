curl -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"state_getMetadata","params":["0x0f50d904467bd3889c9ec795c0e9877b28c559ae171f07fa5781ad1e6ce3b9b4"], "id":1}' \
    https://rpc.helikon.io/polkadot \
  | jq -r .result \
  | sed 's/^0x//' \
  | xxd -r -p > ./polkadot-1004001.scale