# Ferrum Network Bridge - Casper Smart Contracts

This repository contains the smart contracts used for the Bridge Pool on the Casper Network.


This contract has the following functionality:

- add liquidity to the pool
- remove liquidity from the pool
- withdraw liquidity from the pool
- withraw signed liquidity from the pool securely
- swap with another network
- get liquidity already in the pool
- add target for possible swap

## Table of Contents

1. [Getting Started](#getting-started)

2. [Usage](#usage)

3. [Installing and Interacting with the Contract using the Rust Casper Client](#installing-and-interacting-with-the-contract-using-the-rust-casper-client)

4. [Events](#events)

5. [Error Codes](#error-codes)

6. [Contributing](#contributing)

## Getting Started

To get started with using the smart contracts in this repository, you will need to have a working environment for Rust and Casper CLI.

```bash
cargo install casper-client
```

## Usage

### Set up the Rust toolchain

```bash
make prepare
```

### Compile smart contracts

```bash
make build-contract
```

### Run tests

```bash
make test
```

### Installing and Interacting with the Contract using the Rust Casper Client


##### Example deploy

The following is an example of deploying the installation of the contract via the Rust Casper command client.

```bash
casper-client put-deploy \
    --chain-name casper-test \
    --node-address http://44.208.234.65:7777 \
    --secret-key ./contract/keys/secret_key.pem \
    --session-path ./contract/target/wasm32-unknown-unknown/release/bridge_pool.wasm \
    --payment-amount 220000000000
```

##### Example deploy
step3 : add_liquidity
casper-client put-deploy \
    --chain-name casper-test \
    --node-address http://44.208.234.65:7777 \
    --secret-key ./contract/keys/secret_key.pem \
    --session-hash hash-2eaf3bf2cbc8e46f56ce04904592aa530141170fbee3473baeba4edfe9e87513 \
    --session-entry-point add_liquidity \
    --payment-amount 5000000000 \
    --session-arg "amount:u256='20'" \
    --session-arg "token_address:string='contract-package-wasme222974816f70ca96fc4002a696bb552e2959d3463158cd82a7bfc8a94c03473'" \
    --session-arg "bridge_pool_contract_package_hash:string='contract-package-wasm85802b5c7c8a1ebf93fa3d20ed1837e1887c629cc8dc005eb49c64e911cd4abf'"

contract-package-wasmfb784965ea45fbfd24029d2bcab1dc2233e21e3f5975d04ef66db9710ff706d8

##### Example deploy
step3.5 : get_liquidity
casper-client put-deploy \
    --chain-name casper-test \
    --node-address http://44.208.234.65:7777 \
    --secret-key ./contract/keys/secret_key.pem \
    --session-hash hash-2eaf3bf2cbc8e46f56ce04904592aa530141170fbee3473baeba4edfe9e87513 \
    --session-entry-point get_liquidity \
    --payment-amount 5000000000 \
    --session-arg "token_address:string='contract-package-wasme222974816f70ca96fc4002a696bb552e2959d3463158cd82a7bfc8a94c03473'"

##### Example deploy
step4 : remove_liquidity
casper-client put-deploy \
    --chain-name casper-test \
    --node-address http://44.208.234.65:7777 \
    --secret-key ./contract/keys/secret_key.pem \
    --session-hash hash-2eaf3bf2cbc8e46f56ce04904592aa530141170fbee3473baeba4edfe9e87513 \
    --session-entry-point remove_liquidity \
    --payment-amount 5000000000 \
    --session-arg "amount:u256='1'" \
    --session-arg "token_address:string='contract-package-wasme222974816f70ca96fc4002a696bb552e2959d3463158cd82a7bfc8a94c03473'"

##### Example allow_target
```bash
casper-client put-deploy \
    --chain-name casper-test \
    --node-address http://44.208.234.65:7777 \
    --secret-key ./contract/keys/secret_key.pem \
    --session-hash hash-2eaf3bf2cbc8e46f56ce04904592aa530141170fbee3473baeba4edfe9e87513 \
    --session-entry-point allow_target \
    --payment-amount 5000000000 \
    --session-arg "token_address:string='contract-package-wasme222974816f70ca96fc4002a696bb552e2959d3463158cd82a7bfc8a94c03473'" \
    --session-arg "token_name:string='some_unusual_token_name'" \
    --session-arg "target_network:u256='1'" \
    --session-arg "target_token:string='qwe'"
```

##### Example swap
```bash
casper-client put-deploy \
    --chain-name casper-test \
    --node-address http://44.208.234.65:7777 \
    --secret-key ./contract/keys/secret_key.pem \
    --session-hash hash-2eaf3bf2cbc8e46f56ce04904592aa530141170fbee3473baeba4edfe9e87513 \
    --session-entry-point swap \
    --payment-amount 5000000000 \
    --session-arg "amount:u256='1'" \
    --session-arg "target_network:u256='1'" \
    --session-arg "target_token:string='qwe'" \
    --session-arg "token_address:string='contract-package-wasme222974816f70ca96fc4002a696bb552e2959d3463158cd82a7bfc8a94c03473'"
```

##### Example add_signer
```bash
casper-client put-deploy \
    --chain-name casper-test \
    --node-address http://44.208.234.65:7777 \
    --secret-key ./contract/keys/secret_key.pem \
    --session-hash hash-2eaf3bf2cbc8e46f56ce04904592aa530141170fbee3473baeba4edfe9e87513 \
    --session-entry-point add_signer \
    --payment-amount 5000000000 \
    --session-arg "signer:string='cde782dee9643b02dde8a11499ede81ec1d05dd3'"
```

##### Example withdraw_signed
```bash
casper-client put-deploy \
    --chain-name casper-test \
    --node-address http://44.208.234.65:7777 \
    --secret-key /home/ubuntu/release_bridge_pool/bridge-casper-smart-contracts/contract/keys/secret_key.pem \
    --session-hash hash-2eaf3bf2cbc8e46f56ce04904592aa530141170fbee3473baeba4edfe9e87513 \
    --session-entry-point withdraw_signed \
    --payment-amount 505000000000 \
    --session-arg "token_address:string='contract-package-wasme222974816f70ca96fc4002a696bb552e2959d3463158cd82a7bfc8a94c03473'" \
    --session-arg "payee:string='0Bdb79846e8331A19A65430363f240Ec8aCC2A52'" \
    --session-arg "amount:u256='1'" \
    --session-arg "signature:string='b086ec5298630507dc314767a3cdb0d5e38381b11a35096e4f7c8706b51742c100fd299da6b56b33af70482a5656663a3a57d2c52e5442f56d3e948395918f8e1c'" \
    --session-arg "salt:string='6b166cc8016d4ddb7a2578245ac9de73bd95f30ea960ab53dec02141623832dd'" \
    --session-arg "message_hash:string='a02c88bd2abba0d58c72141d00098448a3da586a2f38a3679525a1cbd0fd60d5'"
```

##### Example Withdraw
```bash
casper-client put-deploy \
    --chain-name casper-test \
    --node-address http://44.208.234.65:7777 \
    --secret-key ./contract/keys/secret_key.pem \
    --session-hash hash-2eaf3bf2cbc8e46f56ce04904592aa530141170fbee3473baeba4edfe9e87513 \
    --session-entry-point withdraw \
    --payment-amount 5000000000 \
    --session-arg "amount:u256='1'" \
    --session-arg "token_address:string='contract-package-wasme222974816f70ca96fc4002a696bb552e2959d3463158cd82a7bfc8a94c03473'"
```

## Events

| Event name                | Included values and type                                                                                      |
| ------------------------- | ------------------------------------------------------------------------------------------------------------- |
| BridgeLiquidityAdded      | actor (Key) , token (Key), amount (U256)                                                                      |
| BridgeLiquidityRemoved    | actor (Key) , token (Key), amount (U256)                                                                      |
| BridgeSwap                | actor (Key) , token (Key), target_network: U256, target_token (String) , target_address (Key) , amount (U256) |
| TransferBySignature       | signer (Key), reciever (String), token (Key) , amount (U256)                                                  |


## Error Codes

| Code | Error                                               |
| ---- | --------------------------------------------------- |
| 1    | PermissionDenied                                    |
| 2    | WrongArguments                                      |
| 3    | NotRequiredStake                                    |
| 4    | BadTiming                                           |
| 5    | InvalidContext                                      |
| 6    | NegativeReward                                      |
| 7    | NegativeWithdrawableReward                          |
| 8    | NegativeAmount                                      |
| 9    | MissingContractPackageHash                          |
| 10   | InvalidContractPackageHash                          |
| 11   | InvalidContractHash                                 |
| 12   | WithdrawCheckErrorEarly                             |
| 13   | WithdrawCheckError                                  |
| 14   | NeitherAccountHashNorNeitherContractPackageHash     |
| 15   | UnexpectedContractHash                              |
| 16   | NotContractPackageHash                              |
| 17   | DictTargetTokenNotEqualTargetToken                  |
| 18   | NoTargetNetworkDictForThisToken                     |
| 19   | NoTargetTokenInAllowedTargetsDict                   |
| 20   | ClientDoesNotHaveAnyKindOfLioquidity                |
| 21   | ClientDoesNotHaveSpecificKindOfLioquidity           |
| 22   | AlreadyInThisTargetTokenDict                        |
| 23   | MessageAlreadyUsed                                  |
| 24   | NoValueInSignersDict                                |
| 25   | InvalidSigner                                       |
| 26   | CasperAccountHashParsing                            |
| 27   | WrongTokenName                                      |
| 28   | NoTokenInTokenContractPackageHashDict               |
| 29   | RecoverableSignatureTryFromFail                     |
| 30   | NonRecoverableSignatureTryFromFail                  |
| 31   | RecoverVerifyKeyFail                                |
| 32   | CheckedSubFail                                      |
| 33   | SaltHexFail                                         |
| 34   | SaltWrongSize                                       |
| 35   | SignatureHexFail                                    |
| 36   | NotBridgePoolContractPackageHash                    |
| 37   | EcdsaPublicKeyRecoveryFail                          |
| 38   | MessageHashHexDecodingFail                          |
| 39   | PublicKeyTryIntoFail                                |

## Contributing

If you would like to contribute to this repository, please fork the repository and create a new branch for your changes. Once you have made your changes, submit a pull request and we will review your changes.

Please ensure that your code follows the style and conventions used in the existing codebase, and that it passes all tests before submitting a pull request.