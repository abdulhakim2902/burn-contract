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
near deploy dbio-burn3.testnet out/main.wasm --initFunction new --initArgs ''
```

Get account session:

```bash
near view dbio-burn3.testnet get_account_session '{"token_id":"debio-token4.testnet","account_id":"rumaishakhadijah.testnet"}'
```

Response
```bash
{
  "burn_amount": 100000000000
  "session": 100000000000
}
```

Use session:
```bash
near call dbio-burn3.testnet use_session '{"token_id":"debio-token4.testnet","amount":"1"}' --accountId rumaishakhadijah.testnet
```

Burn:

```bash
near call dbio-burn3.testnet burn '{"token_id":"debio-token4.testnet","amount":"1000000000000000000"}' --accountId rumaishakhadijah.testnet --depositYocto 1
```

ENV
- burn contract: dbio-burn1.testnet, dbio-burn2.testnet,dbio-burn3.testnet
- token contract: debio-token3.testnet, debio-token4.testnet
