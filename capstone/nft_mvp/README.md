# CW721 NFT MVP

## env vars
`export CHAIN_ID="pisco-1"`

`export DENOM="uluna"`

`export BINARY="terrad"`

`export RPC="https://terra-testnet-rpc.polkachu.com:443"`

`export TXFLAG="--node $RPC --chain-id $CHAIN_ID --gas-prices 0.25$DENOM --gas auto --gas-adjustment 1.3 -y -b block --output json"`

## build optimized
`docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.9`

**Xxx**



## x


