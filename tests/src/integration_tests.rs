#[cfg(test)]
mod tests {

    use casper_engine_test_support::{
        ExecuteRequestBuilder, InMemoryWasmTestBuilder, WasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
        DEFAULT_RUN_GENESIS_REQUEST,
    };
    use casper_types::{runtime_args, ContractHash, RuntimeArgs};

    use std::collections::BTreeMap;

    use blake2::{
        digest::{Update, VariableOutput},
        VarBlake2b,
    };
    use casper_execution_engine::storage::global_state::in_memory::InMemoryGlobalState;
    use casper_types::{
        account::AccountHash,
        bytesrepr::{FromBytes, ToBytes},
        CLTyped, ContractPackageHash, Key, URef, BLAKE2B_DIGEST_LENGTH, U256,
    };
    use test_env::TestEnv;

    const ERC20_WASM: &str = "erc20.wasm";
    const BRIDGE_POOL_WASM: &str = "bridge_pool.wasm"; // The main example contract
    const COUNTER_CALL_WASM: &str = "counter-call.wasm"; // The session code that calls the contract
    const ERC20_CONTRACT_NAME: &str = "erc20_token_contract";
    const ERC20_CONTRACT_PACKAGE_HASH: &str = "erc20-contract_package_hash";
    const BRIDGE_POOL_CONTRACT_HASH: &str = "bridge_pool_contract_hash";
    const BRIDGE_POOL_CONTRACT_PACKAGE_HASH: &str = "bridge_pool_package_name";

    const CONTRACT_KEY: &str = "bridge_pool"; // Named key referencing this contract
    const LIQUIDITY_KEY: &str = "liquidity"; // Named key referencing the count value
    const CONTRACT_VERSION_KEY: &str = "version"; // Automatically incremented version in a contract package
    const ALLOWANCES_SEED_UREF: &str = "allowances";

    #[test]
    fn should_be_able_to_install_and_add_liquidity() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

        let erc20_runtime_args = runtime_args! {
            "name" => "FERRUM_ERC20".to_string(),
            "symbol" => "F_ERC20".to_string(),
            "total_supply" => U256::from(500000i64),
            "decimals" => 8u8,
        };

        let erc_20_install_request =
            ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, ERC20_WASM, erc20_runtime_args)
                .build();

        builder
            .exec(erc_20_install_request)
            .expect_success()
            .commit();

        let erc20_contract_hash = get_erc20_contract_hash(&builder);

        println!(
            "erc20_contract_hash {:?}",
            erc20_contract_hash.to_formatted_string()
        );
        let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            erc20_contract_hash,
            "mint",
            runtime_args! {},
        )
        .build();

        builder.exec(mint_request).expect_success().commit();

        let erc20_contract_key: Key = erc20_contract_hash.into();

        let balance = balance_dictionary(
            &builder,
            erc20_contract_key,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
        );
        assert_eq!(balance, U256::from(510000u64));

        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            BRIDGE_POOL_WASM,
            runtime_args! {},
        )
        .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        let bridge_pool_contract_package_hash = get_bridge_pool_contract_package_hash(&builder);

        let bridge_pool_contract_hash = get_bridge_pool_contract_hash(&builder);

        let bridge_pool_contract_key: Key = bridge_pool_contract_package_hash.into();

        let approve_args = runtime_args! {
            "spender" => bridge_pool_contract_key,
            "amount" => U256::from(10i64),
        };

        let approve_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            erc20_contract_hash,
            "approve",
            approve_args,
        )
        .build();

        builder.exec(approve_request).expect_success().commit();

        let actual_allowance = allowance_dictionary(
            &builder,
            erc20_contract_key,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
            bridge_pool_contract_key,
        );

        assert_eq!(actual_allowance, U256::from(10i64));

        let erc20_contract_package_hash = get_erc20_contract_package_hash(&builder);

        println!(
            "erc20_contract_package_hash.to_formatted_string() is {}",
            erc20_contract_package_hash.to_formatted_string()
        );
        println!(
            "bridge_pool_contract_package_hash.to_formatted_string() is {}",
            bridge_pool_contract_package_hash.to_formatted_string()
        );
        println!(
            "bridge_pool_contract_hash.to_formatted_string() is {}",
            bridge_pool_contract_hash.to_formatted_string()
        );

        let erc20_contract_package_hash_string = erc20_contract_package_hash.to_formatted_string();
        let bridge_pool_contract_package_hash_string =
            bridge_pool_contract_package_hash.to_formatted_string();
        let add_liquidity_args = runtime_args! {
            "amount" => U256::from(1i64),
            "token_address" => erc20_contract_package_hash_string,
            "bridge_pool_contract_package_hash" => bridge_pool_contract_package_hash_string ,
        };

        let add_liquidity_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            bridge_pool_contract_hash,
            "add_liquidity",
            add_liquidity_args,
        )
        .build();

        builder
            .exec(add_liquidity_request)
            .expect_success()
            .commit();
    }

    #[test]
    fn should_be_able_to_install_and_get_liquidity() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

        let erc20_runtime_args = runtime_args! {
            "name" => "FERRUM_ERC20".to_string(),
            "symbol" => "F_ERC20".to_string(),
            "total_supply" => U256::from(500000i64),
            "decimals" => 8u8,
        };

        let erc_20_install_request =
            ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, ERC20_WASM, erc20_runtime_args)
                .build();

        builder
            .exec(erc_20_install_request)
            .expect_success()
            .commit();

        let erc20_contract_hash = get_erc20_contract_hash(&builder);

        println!(
            "erc20_contract_hash {:?}",
            erc20_contract_hash.to_formatted_string()
        );
        let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            erc20_contract_hash,
            "mint",
            runtime_args! {},
        )
        .build();

        builder.exec(mint_request).expect_success().commit();

        let erc20_contract_key: Key = erc20_contract_hash.into();

        let balance = balance_dictionary(
            &builder,
            erc20_contract_key,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
        );
        assert_eq!(balance, U256::from(510000u64));

        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            BRIDGE_POOL_WASM,
            runtime_args! {},
        )
        .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        let bridge_pool_contract_package_hash = get_bridge_pool_contract_package_hash(&builder);

        let bridge_pool_contract_hash = get_bridge_pool_contract_hash(&builder);

        let bridge_pool_contract_key: Key = bridge_pool_contract_package_hash.into();

        let approve_args = runtime_args! {
            "spender" => bridge_pool_contract_key,
            "amount" => U256::from(10i64),
        };

        let approve_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            erc20_contract_hash,
            "approve",
            approve_args,
        )
        .build();

        builder.exec(approve_request).expect_success().commit();

        let actual_allowance = allowance_dictionary(
            &builder,
            erc20_contract_key,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
            bridge_pool_contract_key,
        );

        assert_eq!(actual_allowance, U256::from(10i64));

        let erc20_contract_package_hash = get_erc20_contract_package_hash(&builder);

        println!(
            "erc20_contract_package_hash.to_formatted_string() is {}",
            erc20_contract_package_hash.to_formatted_string()
        );
        println!(
            "bridge_pool_contract_package_hash.to_formatted_string() is {}",
            bridge_pool_contract_package_hash.to_formatted_string()
        );
        println!(
            "bridge_pool_contract_hash.to_formatted_string() is {}",
            bridge_pool_contract_hash.to_formatted_string()
        );

        let erc20_contract_package_hash_string = erc20_contract_package_hash.to_formatted_string();
        let bridge_pool_contract_package_hash_string =
            bridge_pool_contract_package_hash.to_formatted_string();

        let add_liquidity_args = runtime_args! {
            "amount" => U256::from(1i64),
            "token_address" => erc20_contract_package_hash_string.clone(),
            "bridge_pool_contract_package_hash" => bridge_pool_contract_package_hash_string ,
        };

        let add_liquidity_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            bridge_pool_contract_hash,
            "add_liquidity",
            add_liquidity_args,
        )
        .build();

        builder
            .exec(add_liquidity_request)
            .expect_success()
            .commit();

        let get_liquidity_args = runtime_args! {
            "token_address" => erc20_contract_package_hash_string,
        };

        let get_liquidity_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            bridge_pool_contract_hash,
            "get_liquidity",
            get_liquidity_args,
        )
        .build();

        builder
            .exec(get_liquidity_request)
            .expect_success()
            .commit();
    }

    #[test]
    fn should_be_able_to_install_and_add_liquidity_and_remove_liquidity() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

        let erc20_runtime_args = runtime_args! {
            "name" => "FERRUM_ERC20".to_string(),
            "symbol" => "F_ERC20".to_string(),
            "total_supply" => U256::from(500000i64),
            "decimals" => 8u8,
        };

        let erc_20_install_request =
            ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, ERC20_WASM, erc20_runtime_args)
                .build();

        builder
            .exec(erc_20_install_request)
            .expect_success()
            .commit();

        let erc20_contract_hash = get_erc20_contract_hash(&builder);

        println!(
            "erc20_contract_hash {:?}",
            erc20_contract_hash.to_formatted_string()
        );
        let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            erc20_contract_hash,
            "mint",
            runtime_args! {},
        )
        .build();

        builder.exec(mint_request).expect_success().commit();

        let erc20_contract_key: Key = erc20_contract_hash.into();

        let balance = balance_dictionary(
            &builder,
            erc20_contract_key,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
        );
        assert_eq!(balance, U256::from(510000u64));

        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            BRIDGE_POOL_WASM,
            runtime_args! {},
        )
        .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        let bridge_pool_contract_package_hash = get_bridge_pool_contract_package_hash(&builder);

        let bridge_pool_contract_hash = get_bridge_pool_contract_hash(&builder);

        let bridge_pool_contract_key: Key = bridge_pool_contract_package_hash.into();

        let approve_args = runtime_args! {
            "spender" => bridge_pool_contract_key,
            "amount" => U256::from(10i64),
        };

        let approve_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            erc20_contract_hash,
            "approve",
            approve_args,
        )
        .build();

        builder.exec(approve_request).expect_success().commit();

        let actual_allowance = allowance_dictionary(
            &builder,
            erc20_contract_key,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
            bridge_pool_contract_key,
        );

        assert_eq!(actual_allowance, U256::from(10i64));

        let erc20_contract_package_hash = get_erc20_contract_package_hash(&builder);

        println!(
            "erc20_contract_package_hash.to_formatted_string() is {}",
            erc20_contract_package_hash.to_formatted_string()
        );
        println!(
            "bridge_pool_contract_package_hash.to_formatted_string() is {}",
            bridge_pool_contract_package_hash.to_formatted_string()
        );
        println!(
            "bridge_pool_contract_hash.to_formatted_string() is {}",
            bridge_pool_contract_hash.to_formatted_string()
        );

        let erc20_contract_package_hash_string = erc20_contract_package_hash.to_formatted_string();
        let bridge_pool_contract_package_hash_string =
            bridge_pool_contract_package_hash.to_formatted_string();

        let add_liquidity_args = runtime_args! {
            "amount" => U256::from(1i64),
            "token_address" => erc20_contract_package_hash_string.clone(),
            "bridge_pool_contract_package_hash" => bridge_pool_contract_package_hash_string ,
        };

        let add_liquidity_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            bridge_pool_contract_hash,
            "add_liquidity",
            add_liquidity_args,
        )
        .build();

        let remove_liquidity_args = runtime_args! {
            "amount" => U256::from(1i64),
            "token_address" => erc20_contract_package_hash_string,
        };

        builder
            .exec(add_liquidity_request)
            .expect_success()
            .commit();

        let remove_liquidity_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            bridge_pool_contract_hash,
            "remove_liquidity",
            remove_liquidity_args,
        )
        .build();

        builder
            .exec(remove_liquidity_request)
            .expect_success()
            .commit();
    }

    #[test]
    fn should_be_able_to_install_and_add_signer() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

        let erc20_runtime_args = runtime_args! {
            "name" => "FERRUM_ERC20".to_string(),
            "symbol" => "F_ERC20".to_string(),
            "total_supply" => U256::from(500000i64),
            "decimals" => 8u8,
        };

        let erc_20_install_request =
            ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, ERC20_WASM, erc20_runtime_args)
                .build();

        builder
            .exec(erc_20_install_request)
            .expect_success()
            .commit();

        let erc20_contract_hash = get_erc20_contract_hash(&builder);

        println!(
            "erc20_contract_hash {:?}",
            erc20_contract_hash.to_formatted_string()
        );
        let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            erc20_contract_hash,
            "mint",
            runtime_args! {},
        )
        .build();

        builder.exec(mint_request).expect_success().commit();

        let erc20_contract_key: Key = erc20_contract_hash.into();

        let balance = balance_dictionary(
            &builder,
            erc20_contract_key,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
        );
        assert_eq!(balance, U256::from(510000u64));

        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            BRIDGE_POOL_WASM,
            runtime_args! {},
        )
        .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        let bridge_pool_contract_package_hash = get_bridge_pool_contract_package_hash(&builder);

        let bridge_pool_contract_hash = get_bridge_pool_contract_hash(&builder);

        let bridge_pool_contract_key: Key = bridge_pool_contract_package_hash.into();

        let approve_args = runtime_args! {
            "spender" => bridge_pool_contract_key,
            "amount" => U256::from(10i64),
        };

        let approve_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            erc20_contract_hash,
            "approve",
            approve_args,
        )
        .build();

        builder.exec(approve_request).expect_success().commit();

        let actual_allowance = allowance_dictionary(
            &builder,
            erc20_contract_key,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
            bridge_pool_contract_key,
        );

        assert_eq!(actual_allowance, U256::from(10i64));

        let erc20_contract_package_hash = get_erc20_contract_package_hash(&builder);

        let erc20_contract_package_hash_string = erc20_contract_package_hash.to_formatted_string();
        let bridge_pool_contract_package_hash_string =
            bridge_pool_contract_package_hash.to_formatted_string();
        let add_signer_args = runtime_args! {
            "signer" => "cde782dee9643b02dde8a11499ede81ec1d05dd3".to_string() ,
        };

        let add_signer_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            bridge_pool_contract_hash,
            "add_signer",
            add_signer_args,
        )
        .build();

        builder.exec(add_signer_request).expect_success().commit();
    }

    #[test]
    fn should_be_able_to_install_add_and_remove_signer() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

        let erc20_runtime_args = runtime_args! {
            "name" => "FERRUM_ERC20".to_string(),
            "symbol" => "F_ERC20".to_string(),
            "total_supply" => U256::from(500000i64),
            "decimals" => 8u8,
        };

        let erc_20_install_request =
            ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, ERC20_WASM, erc20_runtime_args)
                .build();

        builder
            .exec(erc_20_install_request)
            .expect_success()
            .commit();

        let erc20_contract_hash = get_erc20_contract_hash(&builder);

        println!(
            "erc20_contract_hash {:?}",
            erc20_contract_hash.to_formatted_string()
        );
        let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            erc20_contract_hash,
            "mint",
            runtime_args! {},
        )
        .build();

        builder.exec(mint_request).expect_success().commit();

        let erc20_contract_key: Key = erc20_contract_hash.into();

        let balance = balance_dictionary(
            &builder,
            erc20_contract_key,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
        );
        assert_eq!(balance, U256::from(510000u64));

        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            BRIDGE_POOL_WASM,
            runtime_args! {},
        )
        .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        let bridge_pool_contract_package_hash = get_bridge_pool_contract_package_hash(&builder);

        let bridge_pool_contract_hash = get_bridge_pool_contract_hash(&builder);

        let bridge_pool_contract_key: Key = bridge_pool_contract_package_hash.into();

        let approve_args = runtime_args! {
            "spender" => bridge_pool_contract_key,
            "amount" => U256::from(10i64),
        };

        let approve_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            erc20_contract_hash,
            "approve",
            approve_args,
        )
        .build();

        builder.exec(approve_request).expect_success().commit();

        let actual_allowance = allowance_dictionary(
            &builder,
            erc20_contract_key,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
            bridge_pool_contract_key,
        );

        assert_eq!(actual_allowance, U256::from(10i64));

        let erc20_contract_package_hash = get_erc20_contract_package_hash(&builder);

        let erc20_contract_package_hash_string = erc20_contract_package_hash.to_formatted_string();
        let bridge_pool_contract_package_hash_string =
            bridge_pool_contract_package_hash.to_formatted_string();
        let add_signer_args = runtime_args! {
            "signer" => "cde782dee9643b02dde8a11499ede81ec1d05dd3".to_string() ,
        };

        let add_signer_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            bridge_pool_contract_hash,
            "add_signer",
            add_signer_args,
        )
        .build();

        builder.exec(add_signer_request).expect_success().commit();

        let remove_signer_args = runtime_args! {
            "signer" => "cde782dee9643b02dde8a11499ede81ec1d05dd3".to_string() ,
        };

        let remove_signer_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            bridge_pool_contract_hash,
            "remove_signer",
            remove_signer_args,
        )
        .build();

        builder.exec(remove_signer_request).expect_success().commit();
    }

    #[test]
    fn should_be_able_to_install_and_add_liquidity_and_withdraw_signed() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

        let erc20_runtime_args = runtime_args! {
            "name" => "FERRUM_ERC20".to_string(),
            "symbol" => "F_ERC20".to_string(),
            "total_supply" => U256::from(500000i64),
            "decimals" => 8u8,
        };

        let erc_20_install_request =
            ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, ERC20_WASM, erc20_runtime_args)
                .build();

        builder
            .exec(erc_20_install_request)
            .expect_success()
            .commit();

        let erc20_contract_hash = get_erc20_contract_hash(&builder);

        println!(
            "erc20_contract_hash {:?}",
            erc20_contract_hash.to_formatted_string()
        );
        let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            erc20_contract_hash,
            "mint",
            runtime_args! {},
        )
        .build();

        builder.exec(mint_request).expect_success().commit();

        let erc20_contract_key: Key = erc20_contract_hash.into();

        let balance = balance_dictionary(
            &builder,
            erc20_contract_key,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
        );
        assert_eq!(balance, U256::from(510000u64));

        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            BRIDGE_POOL_WASM,
            runtime_args! {},
        )
        .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        let bridge_pool_contract_package_hash = get_bridge_pool_contract_package_hash(&builder);

        let bridge_pool_contract_hash = get_bridge_pool_contract_hash(&builder);

        let bridge_pool_contract_key: Key = bridge_pool_contract_package_hash.into();

        let approve_args = runtime_args! {
            "spender" => bridge_pool_contract_key,
            "amount" => U256::from(10i64),
        };

        let approve_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            erc20_contract_hash,
            "approve",
            approve_args,
        )
        .build();

        builder.exec(approve_request).expect_success().commit();

        let actual_allowance = allowance_dictionary(
            &builder,
            erc20_contract_key,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
            bridge_pool_contract_key,
        );

        assert_eq!(actual_allowance, U256::from(10i64));

        let erc20_contract_package_hash = get_erc20_contract_package_hash(&builder);

        println!(
            "erc20_contract_package_hash.to_formatted_string() is {}",
            erc20_contract_package_hash.to_formatted_string()
        );
        println!(
            "bridge_pool_contract_package_hash.to_formatted_string() is {}",
            bridge_pool_contract_package_hash.to_formatted_string()
        );
        println!(
            "bridge_pool_contract_hash.to_formatted_string() is {}",
            bridge_pool_contract_hash.to_formatted_string()
        );

        let erc20_contract_package_hash_string = erc20_contract_package_hash.to_formatted_string();
        let bridge_pool_contract_package_hash_string =
            bridge_pool_contract_package_hash.to_formatted_string();

        let add_liquidity_args = runtime_args! {
            "amount" => U256::from(1i64),
            "token_address" => erc20_contract_package_hash_string.clone(),
            "bridge_pool_contract_package_hash" => bridge_pool_contract_package_hash_string ,
        };

        let add_liquidity_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            bridge_pool_contract_hash,
            "add_liquidity",
            add_liquidity_args,
        )
        .build();

        let withdraw_signed_args = runtime_args! {
            "amount" => U256::from(1i64),
            "token_address" => erc20_contract_package_hash_string,
            "payee" => "0Bdb79846e8331A19A65430363f240Ec8aCC2A52".to_string(),
            "signature" => "b086ec5298630507dc314767a3cdb0d5e38381b11a35096e4f7c8706b51742c100fd299da6b56b33af70482a5656663a3a57d2c52e5442f56d3e948395918f8e1c".to_string(),
            "salt" => "6b166cc8016d4ddb7a2578245ac9de73bd95f30ea960ab53dec02141623832dd".to_string(),
            "message_hash" => "a02c88bd2abba0d58c72141d00098448a3da586a2f38a3679525a1cbd0fd60d5".to_string(),
        };

        builder
            .exec(add_liquidity_request)
            .expect_success()
            .commit();

        let add_signer_args = runtime_args! {
            "signer" => "cde782dee9643b02dde8a11499ede81ec1d05dd3".to_string() ,
        };

        let add_signer_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            bridge_pool_contract_hash,
            "add_signer",
            add_signer_args,
        )
        .build();

        builder.exec(add_signer_request).expect_success().commit();

        let withdraw_signed_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            bridge_pool_contract_hash,
            "withdraw_signed",
            withdraw_signed_args,
        )
        .build();

        builder
            .exec(withdraw_signed_request)
            .expect_success()
            .commit();
    }

    #[test]
    fn should_be_able_to_install_and_allow_target() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

        let erc20_runtime_args = runtime_args! {
            "name" => "FERRUM_ERC20".to_string(),
            "symbol" => "F_ERC20".to_string(),
            "total_supply" => U256::from(500000i64),
            "decimals" => 8u8,
        };

        let erc_20_install_request =
            ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, ERC20_WASM, erc20_runtime_args)
                .build();

        builder
            .exec(erc_20_install_request)
            .expect_success()
            .commit();

        let erc20_contract_hash = get_erc20_contract_hash(&builder);

        println!(
            "erc20_contract_hash {:?}",
            erc20_contract_hash.to_formatted_string()
        );
        let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            erc20_contract_hash,
            "mint",
            runtime_args! {},
        )
        .build();

        builder.exec(mint_request).expect_success().commit();

        let erc20_contract_key: Key = erc20_contract_hash.into();

        let balance = balance_dictionary(
            &builder,
            erc20_contract_key,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
        );
        assert_eq!(balance, U256::from(510000u64));

        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            BRIDGE_POOL_WASM,
            runtime_args! {},
        )
        .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        let bridge_pool_contract_package_hash = get_bridge_pool_contract_package_hash(&builder);

        let bridge_pool_contract_hash = get_bridge_pool_contract_hash(&builder);

        let bridge_pool_contract_key: Key = bridge_pool_contract_package_hash.into();

        let approve_args = runtime_args! {
            "spender" => bridge_pool_contract_key,
            "amount" => U256::from(10i64),
        };

        let approve_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            erc20_contract_hash,
            "approve",
            approve_args,
        )
        .build();

        builder.exec(approve_request).expect_success().commit();

        let actual_allowance = allowance_dictionary(
            &builder,
            erc20_contract_key,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
            bridge_pool_contract_key,
        );

        assert_eq!(actual_allowance, U256::from(10i64));

        let erc20_contract_package_hash = get_erc20_contract_package_hash(&builder);

        let erc20_contract_package_hash_string = erc20_contract_package_hash.to_formatted_string();
        let bridge_pool_contract_package_hash_string =
            bridge_pool_contract_package_hash.to_formatted_string();
        let allow_target_args = runtime_args! {
            "token_address" => "contract-package-wasme222974816f70ca96fc4002a696bb552e2959d3463158cd82a7bfc8a94c03473".to_string() ,
            "token_name" => "some_unusual_token_name".to_string() ,
            "target_token" => "qwe".to_string() ,
            "target_network" => U256::from(1i64),
        };

        let allow_target_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            bridge_pool_contract_hash,
            "allow_target",
            allow_target_args,
        )
        .build();

        builder.exec(allow_target_request).expect_success().commit();
    }

    #[test]
    fn should_be_able_to_install_and_add_liquidity_and_swap() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

        let erc20_runtime_args = runtime_args! {
            "name" => "FERRUM_ERC20".to_string(),
            "symbol" => "F_ERC20".to_string(),
            "total_supply" => U256::from(500000i64),
            "decimals" => 8u8,
        };

        let erc_20_install_request =
            ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, ERC20_WASM, erc20_runtime_args)
                .build();

        builder
            .exec(erc_20_install_request)
            .expect_success()
            .commit();

        let erc20_contract_hash = get_erc20_contract_hash(&builder);

        println!(
            "erc20_contract_hash {:?}",
            erc20_contract_hash.to_formatted_string()
        );
        let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            erc20_contract_hash,
            "mint",
            runtime_args! {},
        )
        .build();

        builder.exec(mint_request).expect_success().commit();

        let erc20_contract_key: Key = erc20_contract_hash.into();

        let balance = balance_dictionary(
            &builder,
            erc20_contract_key,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
        );
        assert_eq!(balance, U256::from(510000u64));

        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            BRIDGE_POOL_WASM,
            runtime_args! {},
        )
        .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        let bridge_pool_contract_package_hash = get_bridge_pool_contract_package_hash(&builder);

        let bridge_pool_contract_hash = get_bridge_pool_contract_hash(&builder);

        let bridge_pool_contract_key: Key = bridge_pool_contract_package_hash.into();

        let approve_args = runtime_args! {
            "spender" => bridge_pool_contract_key,
            "amount" => U256::from(10i64),
        };

        let approve_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            erc20_contract_hash,
            "approve",
            approve_args,
        )
        .build();

        builder.exec(approve_request).expect_success().commit();

        let actual_allowance = allowance_dictionary(
            &builder,
            erc20_contract_key,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
            bridge_pool_contract_key,
        );

        assert_eq!(actual_allowance, U256::from(10i64));

        let erc20_contract_package_hash = get_erc20_contract_package_hash(&builder);

        let erc20_contract_package_hash_string = erc20_contract_package_hash.to_formatted_string();
        let bridge_pool_contract_package_hash_string =
            bridge_pool_contract_package_hash.to_formatted_string();

        println!(
            "erc20_contract_package_hash.to_formatted_string() is {}",
            erc20_contract_package_hash.to_formatted_string()
        );
        println!(
            "bridge_pool_contract_package_hash.to_formatted_string() is {}",
            bridge_pool_contract_package_hash.to_formatted_string()
        );
        println!(
            "bridge_pool_contract_hash.to_formatted_string() is {}",
            bridge_pool_contract_hash.to_formatted_string()
        );

        let add_liquidity_args = runtime_args! {
            "amount" => U256::from(1i64),
            "token_address" => erc20_contract_package_hash_string,
            "bridge_pool_contract_package_hash" => bridge_pool_contract_package_hash_string ,
        };

        let add_liquidity_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            bridge_pool_contract_hash,
            "add_liquidity",
            add_liquidity_args,
        )
        .build();

        builder
            .exec(add_liquidity_request)
            .expect_success()
            .commit();

        let allow_target_args = runtime_args! {
            "token_address" => erc20_contract_package_hash.to_formatted_string(),
            "token_name" => "some_unusual_token_name".to_string() ,
            "target_token" => "qwe".to_string() ,
            "target_network" => U256::from(1i64),
        };

        let allow_target_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            bridge_pool_contract_hash,
            "allow_target",
            allow_target_args,
        )
        .build();

        builder.exec(allow_target_request).expect_success().commit();

        let swap_args = runtime_args! {
            "token_address" => erc20_contract_package_hash.to_formatted_string(),
            "target_token" => "qwe".to_string() ,
            "target_network" => U256::from(1i64),
            "amount" => U256::from(1i64),
        };

        let swap_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            bridge_pool_contract_hash,
            "swap",
            swap_args,
        )
        .build();

        builder.exec(swap_request).expect_success().commit();
    }

    /// Creates a dictionary item key for an (owner, spender) pair.
    fn make_allowances_dictionary_item_key(owner: Key, spender: Key) -> String {
        let mut preimage = Vec::new();
        preimage.append(&mut owner.to_bytes().unwrap());
        preimage.append(&mut spender.to_bytes().unwrap());

        let key_bytes = create_blake2b_hash(&preimage);
        hex::encode(&key_bytes)
    }

    pub(crate) fn create_blake2b_hash<T: AsRef<[u8]>>(data: T) -> [u8; BLAKE2B_DIGEST_LENGTH] {
        let mut result = [0; BLAKE2B_DIGEST_LENGTH];
        // NOTE: Assumed safe as `BLAKE2B_DIGEST_LENGTH` is a valid value for a hasher
        let mut hasher = VarBlake2b::new(BLAKE2B_DIGEST_LENGTH).expect("should create hasher");

        hasher.update(data);
        hasher.finalize_variable(|slice| {
            result.copy_from_slice(slice);
        });
        result
    }

    pub fn get_bridge_pool_contract_package_hash(
        builder: &WasmTestBuilder<InMemoryGlobalState>,
    ) -> ContractPackageHash {
        let bridge_pool_hash_addr = builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .get(BRIDGE_POOL_CONTRACT_PACKAGE_HASH)
            .expect("must have this entry in named keys")
            .into_hash()
            .expect("must get hash_addr");

        ContractPackageHash::new(bridge_pool_hash_addr)
    }

    pub fn get_erc20_contract_package_hash(
        builder: &WasmTestBuilder<InMemoryGlobalState>,
    ) -> ContractPackageHash {
        let erc20_hash_addr = builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .get(ERC20_CONTRACT_PACKAGE_HASH)
            .expect("must have this entry in named keys")
            .into_hash()
            .expect("must get hash_addr");

        ContractPackageHash::new(erc20_hash_addr)
    }

    pub fn get_bridge_pool_contract_hash(
        builder: &WasmTestBuilder<InMemoryGlobalState>,
    ) -> ContractHash {
        let bridge_pool_hash_addr = builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .get(BRIDGE_POOL_CONTRACT_HASH)
            .expect("must have this entry in named keys")
            .into_hash()
            .expect("must get hash_addr");

        ContractHash::new(bridge_pool_hash_addr)
    }

    pub(crate) fn get_erc20_contract_hash(
        builder: &WasmTestBuilder<InMemoryGlobalState>,
    ) -> ContractHash {
        let erc20_hash_addr = builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .get(ERC20_CONTRACT_NAME)
            .expect("must have this entry in named keys")
            .into_hash()
            .expect("must get hash_addr");

        ContractHash::new(erc20_hash_addr)
    }

    fn balance_dictionary(
        builder: &WasmTestBuilder<InMemoryGlobalState>,
        erc20_contract_key: Key,
        owner_key: Key,
    ) -> U256 {
        let balance_seed_uref = builder
            .query(None, erc20_contract_key, &vec![])
            .unwrap()
            .as_contract()
            .expect("must have ERC20 contract")
            .named_keys()
            .get("balances")
            .expect("must have balances entry")
            .as_uref()
            .expect("must be a uref")
            .to_owned();

        let dict_item_key = make_dictionary_item_key(owner_key);

        let balance = builder
            .query_dictionary_item(None, balance_seed_uref, &dict_item_key)
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t()
            .expect("must convert to U256");

        balance
    }

    fn allowance_dictionary(
        builder: &WasmTestBuilder<InMemoryGlobalState>,
        erc20_contract_key: Key,
        owner_key: Key,
        spender_key: Key,
    ) -> U256 {
        let allowance_seed_uref = builder
            .query(None, erc20_contract_key, &vec![])
            .unwrap()
            .as_contract()
            .expect("must have ERC20 contract")
            .named_keys()
            .get(ALLOWANCES_SEED_UREF)
            .expect("must have allowances entry")
            .as_uref()
            .expect("must be a uref")
            .to_owned();

        let dict_item_key = make_allowances_dictionary_item_key(owner_key, spender_key);

        let allowance = builder
            .query_dictionary_item(None, allowance_seed_uref, &dict_item_key)
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t()
            .expect("must convert to U256");

        allowance
    }

    fn make_dictionary_item_key(owner: Key) -> String {
        let preimage = owner.to_bytes().unwrap();
        // NOTE: As for now dictionary item keys are limited to 64 characters only. Instead of using
        // hashing (which will effectively hash a hash) we'll use base64. Preimage is about 33 bytes for
        // both Address variants, and approximated base64-encoded length will be 4 * (33 / 3) ~ 44
        // characters.
        // Even if the preimage increased in size we still have extra space but even in case of much
        // larger preimage we can switch to base85 which has ratio of 4:5.
        base64::encode(&preimage)
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
