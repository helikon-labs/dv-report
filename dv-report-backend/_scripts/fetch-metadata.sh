curl -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"state_getMetadata","params":["0x5cc15be6c5e5ec1ad4f5edfb96d3aed66ec31d4030c4fdaeb494ce8abceaf03d"], "id":1}' \
    https://rpc.helikon.io/kusama \
  | jq -r .result \
  | sed 's/^0x//' \
  | xxd -r -p > ./kusama.scale