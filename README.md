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
./scripts/deploy.sh dbio-burn5.testnet testnet
```

### Get account conversation balance:

```bash
near view dbio-burn5.testnet balance_of '{"token_id":"debio-token5.testnet","account_id":"rumaishakhadijah.testnet"}'
```

Response
```bash
10
```

### Converse:
```bash
near call dbio-burn5.testnet converse '{"token_id":"debio-token5.testnet","amount":"1000000000000000000"}' --accountId rumaishakhadijah.testnet
```

### Burn token:

```bash
```bash
near call debio-token5.testnet storage_deposit '{"account_id":"dbio-burn5.testnet"}' --accountId rumaishakhadijah.testnet --deposit 0.00125
near call debio-token5.testnet ft_transfer_call '{"receiver_id":"dbio-burn5.testnet", "amount": "1000000000000000000", "msg": ""}' --accountId rumaishakhadijah.testnet --depositYocto 1 --gas 300000000000000
near call dbio-burn5.testnet burn '{"token_id":"debio-token5.testnet"}' --accountId rumaishakhadijah.testnet --depositYocto 1
```

Testnet
- burn contract: dbio-burn5.testnet
- token contract: debio-token5.testnet
