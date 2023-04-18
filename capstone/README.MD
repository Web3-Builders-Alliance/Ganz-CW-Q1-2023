# Capstone GenMage NFT MVP

## Store and Initialize CW20 Contract

`terrad tx wasm store ./artifacts/cw20_base.wasm --from wsl_wba $TXFLAG -y -b block`

`export CODE_ID=8607`

`
INIT='{"name":"Imgcoin","symbol":"IMG","decimals":6,"initial_balances":[{"address":"terra1dualq0yjg23mzd9aqr3jvgr3wy9decap6uj8zg","amount":"10000"}], "mint":{"minter":"terra1dualq0yjg23mzd9aqr3jvgr3wy9decap6uj8zg"},"marketing":{}}'
`

`terrad tx wasm instantiate $CODE_ID "$INIT" --label="CW20    Contract" --from wsl_wba $TXFLAG -y --output json -b block --no-admin`

### the cw20 contract address
`export CA="terra1za8crazxwd7qpvazcqtpg2s4rpm7atnyuhjv5j7f8mtlqw47yuwqdz68wd"`


## Store CW721 Contract

`terrad tx wasm store ./artifacts/nft_mvp.wasm --from wsl_wba $TXFLAG -y -b block`

`export CODE_ID=8640`

## Initialize CW721 Contract
`INIT='{"owner": "terra1dualq0yjg23mzd9aqr3jvgr3wy9decap6uj8zg","max_tokens": "1000", "unit_price": "10", "name": "GenMage", "symbol": "GMG", "token_code_id": "1", "cw20_address": "terra1za8crazxwd7qpvazcqtpg2s4rpm7atnyuhjv5j7f8mtlqw47yuwqdz68wd", "token_uri": "https://res.cloudinary.com", "extension": ""}'`

`terrad tx wasm instantiate $CODE_ID "$INIT" --label="NFT MVP Contract" --from wsl_wba $TXFLAG -y --output json -b block --no-admin`

    Error: rpc error: code = Unknown desc = rpc error: code = Unknown desc = failed to execute message; message index: 0: 
    Error parsing into type nft_mvp::msg::InstantiateMsg: Invalid type: instantiate wasm contract failed 
    [CosmWasm/wasmd@v0.30.0/x/wasm/keeper/keeper.go:360] With gas wanted: '50000000' and gas used: '130346' : unknown request



**TODO**




