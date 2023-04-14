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
    const BRIDGE_POOL_CONTRACT_PACKAGE_HASH: &str = "bridge_pool_package_hash";

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

        let contract_hash = builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .get(CONTRACT_KEY)
            .expect("must have contract hash key as part of contract creation")
            .into_hash()
            .map(ContractHash::new)
            .expect("must get contract hash");

        // Verify the first contract version is 1. We'll check this when we upgrade later

        let account = builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("should have account");

        let version_key = *account
            .named_keys()
            .get(CONTRACT_VERSION_KEY)
            .expect("version uref should exist");

        let version = builder
            .query(None, version_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<u32>()
            .expect("should be u32.");

        assert_eq!(version, 1);

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

        dbg!(builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys());

        builder
            .exec(add_liquidity_request)
            .expect_success()
            .commit();

        dbg!(builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys());
    }

    // #[test]
    fn should_be_able_to_install_and_add_liquidity_and_remove_liquidity() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST).commit();

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

        let contract_hash = builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .get(CONTRACT_KEY)
            .expect("must have contract hash key as part of contract creation")
            .into_hash()
            .map(ContractHash::new)
            .expect("must get contract hash");

        // Verify the first contract version is 1. We'll check this when we upgrade later

        let account = builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("should have account");

        let version_key = *account
            .named_keys()
            .get(CONTRACT_VERSION_KEY)
            .expect("version uref should exist");

        let version = builder
            .query(None, version_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<u32>()
            .expect("should be u32.");

        assert_eq!(version, 1);

        // Verify the initial value of count is 0

        let contract = builder
            .get_contract(contract_hash)
            .expect("this contract should exist");

        let liquidity_key = *contract
            .named_keys()
            .get(LIQUIDITY_KEY)
            .expect("count uref should exis in the contract named keys");

        let count = builder
            .query(None, liquidity_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<i32>()
            .expect("should be i32.");

        assert_eq!(count, 0);

        // Use session code to increment the counter

        let session_code_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            COUNTER_CALL_WASM,
            runtime_args! {
                CONTRACT_KEY => contract_hash
            },
        )
        .build();

        builder.exec(session_code_request).expect_success().commit();

        let incremented_count = builder
            .query(None, liquidity_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<i32>()
            .expect("should be i32.");

        assert_eq!(incremented_count, 1);
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
