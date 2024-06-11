# burn-token

This contract is used for burning the token (reducing the total supply) without delete the account

## Building

To build run:

```shell
./build.sh
```

## Deploy

To deploy run:

```shell
near deploy dbio-burn1.testnet out/main.wasm --initFunction new --initArgs '{"token_id": "debio-token3.testnet"}'
```

Get account session:

```bash
near view dbio-burn1.testnet get_account_session '{"account_id": "abdulhakim.tesnet"}'
```

Response
```bash
{
  "amount": "100000000000", // total burn
  "session": "100000000000" // total session
}
```

Get token id:
```bash
near view dbio-burn1.testnet get_token_id
```
Response
```bash
dbio-burn1.tesnet
```

Burn
```bash
near call dbio-burn1.testnet burn '{"amount": "10000"}' --accountId abdulhakim.testnet --depositYocto 1
```

ENV
- burn contract: dbio-burn1.testnet
- token contract: debio-token3.testnet
