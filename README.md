# burn-token

This contract is used for burning the token (reducing the total supply) without delete the account

## Installing
Please install near cli here.
[NEAR CLI](ttps://docs.near.org/tools/near-cli#near-create-account)

## Building

To build run:

```shell
./scripts/build.sh
```

## Deploy

To deploy run:

```shell
./scripts/deploy.sh $contract $near_env
```
Example:
```shell
./scripts/deploy.sh dbio-burn3.testnet testnet
```

### Get account conversation balance:

```bash
near view dbio-burn3.testnet balance_of '{"token_id":"debio-token4.testnet","account_id":"rumaishakhadijah.testnet"}'
```

Response
```bash
10
```

### Converse:
```bash
near call dbio-burn3.testnet converse '{"token_id":"debio-token4.testnet","amount":"1"}' --accountId rumaishakhadijah.testnet
```

### Burn token:

```bash
near call dbio-burn3.testnet burn '{"token_id":"debio-token4.testnet","amount":"1000000000000000000"}' --accountId rumaishakhadijah.testnet --depositYocto 1
```

Testnet
- burn contract: dbio-burn5.testnet
- token contract: debio-token4.testnet
