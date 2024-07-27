# burn-token

This contract is used for burning the token (reducing the total supply) without delete the account

## Building

To build run:

```shell
./scripts/build.sh
```

## Deploy

To deploy run:

```shell
near deploy dbio-burn3.testnet out/burn_token.wasm --initFunction new --initArgs '{}'
```

Get account conversation balance:

```bash
near view dbio-burn3.testnet balance_of '{"token_id":"debio-token4.testnet","account_id":"rumaishakhadijah.testnet"}'
```

Response
```bash
10
```

Converse:
```bash
near call dbio-burn3.testnet converse '{"token_id":"debio-token4.testnet","amount":"1"}' --accountId rumaishakhadijah.testnet
```

Burn:

```bash
near call dbio-burn3.testnet burn '{"token_id":"debio-token4.testnet","amount":"1000000000000000000"}' --accountId rumaishakhadijah.testnet --depositYocto 1
```

Testnet
- burn contract: dbio-burn5.testnet
- token contract: debio-token4.testnet

