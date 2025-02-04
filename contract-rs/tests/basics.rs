use near_sdk::{borsh, AccountId, Gas, NearToken};
use near_workspaces::{
    types::{KeyType, SecretKey},
    Account, BlockHeight,
};
use serde_json::json;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[tokio::test]
async fn create_dev_account() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let dev_account = sandbox.dev_create_account().await?;

    println!("dev_account: {:#?}", dev_account);
    // dev_account: Account { id: AccountId("dev-20250131121318-56660632612680") }

    assert_eq!(
        type_of(dev_account),
        "near_workspaces::types::account::Account"
    );

    Ok(())
}

#[tokio::test]
async fn create_subaccount() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let root_account = sandbox.root_account().unwrap();

    println!("root_account: {:#?}", root_account);
    // root_account: Account { id: AccountId("test.near") }

    let alice_account = root_account
        .create_subaccount("alice")
        .initial_balance(NearToken::from_near(1))
        .transact()
        .await?
        .into_result()?;

    println!("alice_account: {:#?}", alice_account);
    // alice_account: Account { id: AccountId("alice.test.near") }

    assert_eq!(alice_account.id(), "alice.test.near");

    Ok(())
}

#[tokio::test]
async fn create_account_using_secret_key() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let account_id: AccountId = "test-account.near".parse().unwrap();
    let secret_key = SecretKey::from_random(KeyType::ED25519);

    let account = Account::from_secret_key(account_id, secret_key, &sandbox);

    println!("account: {:#?}", account);
    // account: Account { id: AccountId("test-account.near") }

    assert_eq!(account.id(), "test-account.near");

    Ok(())
}

#[tokio::test]
async fn create_account_from_file() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let account = Account::from_file("./tests/credentials.json", &sandbox)?;

    println!("account: {:#?}", account);
    // account: Account { id: AccountId("test-ac-1719933221123-3.testnet") }

    assert_eq!(account.id(), "test-ac-1719933221123-3.testnet");

    Ok(())
}

#[tokio::test]
async fn compile_wasm_file() -> Result<(), Box<dyn std::error::Error>> {
    let compiling_wasm_result = near_workspaces::compile_project("./").await;

    assert!(compiling_wasm_result.is_ok());

    let _contract_wasm =
        compiling_wasm_result.expect("Could not process WASM file after compiling");

    Ok(())
}

#[tokio::test]
async fn load_wasm_from_file() -> Result<(), Box<dyn std::error::Error>> {
    let artifact_path = "target/near/hello_rs.wasm";

    let reading_file_result = std::fs::read(artifact_path);

    assert!(reading_file_result.is_ok());

    let _contract_wasm = reading_file_result
        .expect(format!("Could not read WASM file from {}", artifact_path).as_str());

    Ok(())
}

#[tokio::test]
async fn dev_deploy() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let wasm_file = near_workspaces::compile_project("./").await?;

    let contract = sandbox.dev_deploy(&wasm_file).await?;

    println!("contract: {:#?}", contract);
    // contract: Contract { id: AccountId("dev-20250131125513-33446418241044") }

    assert_eq!(
        type_of(contract),
        "near_workspaces::types::account::Contract"
    );

    Ok(())
}

#[tokio::test]
async fn deploy_to_account() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let wasm_file = near_workspaces::compile_project("./").await?;
    let account = sandbox.dev_create_account().await?;

    let contract = account.deploy(&wasm_file).await?.unwrap();

    println!("contract: {:#?}", contract);
    // contract: Contract { id: AccountId("dev-20250131125513-33446418241044") }

    assert_eq!(
        type_of(contract),
        "near_workspaces::types::account::Contract"
    );

    Ok(())
}

#[tokio::test]
async fn get_account_balance() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let account = sandbox.dev_create_account().await?;

    let account_details = account
        .view_account()
        .await
        .expect("Account has to have some balance");

    println!("account_details: {:#?}", account_details);
    // account_details: AccountDetails { balance: NearToken { inner: 100000000000000000000000000 }, locked: NearToken { inner: 0 }, code_hash: 11111111111111111111111111111111, storage_usage: 182, storage_paid_at: 0 }

    assert_eq!(account_details.balance, NearToken::from_near(100));

    Ok(())
}

#[tokio::test]
async fn call_transaction() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let ft_wasm = near_workspaces::compile_project("./tests/contracts/ft").await?;
    let ft_contract = sandbox.dev_deploy(&ft_wasm).await?;

    let initialize_result = ft_contract
        .call("new_default_meta")
        .args_json(json!({"owner_id": ft_contract.id(), "name": "token", "symbol": "tt", "total_supply": "1000000000000000000000000" }))
        .gas(Gas::from_tgas(100))
        .transact()
        .await?;

    println!("initialize_result: {:#?}", initialize_result);
    // initialize_result: ExecutionFinalResult { total_gas_burnt: NearGas { inner: 2225558148031 }, transaction: ExecutionOutcome { transaction_hash: 9NhQxh78Q7bdnYeYr7ccmhkb9z9iBnyf9JyoL4PF5uUb, block_hash: 4iQtc95L2SmyoU3XdxtcHQQURzbQJod6VFSgYZvYKwBy, logs: [], receipt_ids: [3qtZNe36azn73j2ndQNWNL4kDEeMBn3fQiBunAokoaop], gas_burnt: NearGas { inner: 308363587024 }, tokens_burnt: NearToken { inner: 30836358702400000000 }, executor_id: AccountId("dev-20250131130629-49143087716943"), status: SuccessReceiptId(3qtZNe36azn73j2ndQNWNL4kDEeMBn3fQiBunAokoaop) }, receipts: [ExecutionOutcome { transaction_hash: 3qtZNe36azn73j2ndQNWNL4kDEeMBn3fQiBunAokoaop, block_hash: 4iQtc95L2SmyoU3XdxtcHQQURzbQJod6VFSgYZvYKwBy, logs: ["EVENT_JSON:{\"standard\":\"nep141\",\"version\":\"1.0.0\",\"event\":\"ft_mint\",\"data\":[{\"owner_id\":\"dev-20250131130629-49143087716943\",\"amount\":\"1000000000000000000000000\",\"memo\":\"new tokens are minted\"}]}"], receipt_ids: [FrSvWZWMUCZbiuGrmkj19UTciXMTZgenZmJmLkUPt1AK], gas_burnt: NearGas { inner: 1917194561007 }, tokens_burnt: NearToken { inner: 191719456100700000000 }, executor_id: AccountId("dev-20250131130629-49143087716943"), status: SuccessValue('') }], status: SuccessValue('') }

    assert!(initialize_result.is_success());

    Ok(())
}

#[tokio::test]
async fn view_transaction() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let ft_wasm = near_workspaces::compile_project("./tests/contracts/ft").await?;
    let ft_contract = sandbox.dev_deploy(&ft_wasm).await?;

    let initialize_result = ft_contract
        .call("new_default_meta")
        .args_json(json!({"owner_id": ft_contract.id(), "name": "token", "symbol": "tt", "total_supply": "1000000000000000000000000" }))
        .gas(Gas::from_tgas(100))
        .transact()
        .await?;
    assert!(initialize_result.is_success());

    let view_transaction_result = ft_contract
        .call("ft_balance_of")
        .args_json((ft_contract.id(),))
        .view()
        .await?;

    println!("view_transaction_result: {:#?}", view_transaction_result);
    // ViewResultDetails { result: [34, 49, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 34], logs: [] }

    let account_balance = view_transaction_result.json::<NearToken>()?;

    println!("account_balance: {:#?}", account_balance);
    // account_details: AccountDetails { balance: NearToken { inner: 100000000000000000000000000 }, locked: NearToken { inner: 0 }, code_hash: 11111111111111111111111111111111, storage_usage: 182, storage_paid_at: 0 }

    assert_eq!(account_balance, NearToken::from_near(1));

    Ok(())
}

#[tokio::test]
async fn patch_state() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let wasm_file = near_workspaces::compile_project("./").await?;
    let contract = sandbox.dev_deploy(&wasm_file).await?;

    let new_greeting = "Howdy";

    let _ = sandbox
        .patch_state(
            contract.id(),
            "STATE".as_bytes(),
            &borsh::to_vec(new_greeting)?,
        )
        .await?;

    let current_greeting = contract
        .call("get_greeting")
        .view()
        .await?
        .json::<String>()?;

    println!("current_greeting: {:#?}", current_greeting);
    // current_greeting: "Howdy"

    assert_eq!(current_greeting, new_greeting);

    Ok(())
}

#[tokio::test]
async fn time_travel() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let wasm_file = near_workspaces::compile_project("./tests/contracts/simple-contract").await?;
    let contract = sandbox.dev_deploy(&wasm_file).await?;

    let (timestamp, epoch_height): (u64, u64) =
        contract.call("current_env_data").view().await?.json()?;

    println!("timestamp = {}, epoch_height = {}", timestamp, epoch_height);

    let block_info = sandbox.view_block().await?;

    println!("BlockInfo pre-fast_forward {:#?}", block_info);

    // Call into fast_forward. This will take a bit of time to invoke, but is
    // faster than manually waiting for the same amounts of blocks to be produced
    sandbox.fast_forward(10000).await?;

    let (timestamp, epoch_height): (u64, u64) =
        contract.call("current_env_data").view().await?.json()?;

    println!("timestamp = {}, epoch_height = {}", timestamp, epoch_height);

    let block_info = sandbox.view_block().await?;

    println!("BlockInfo post-fast_forward {:#?}", block_info);

    Ok(())
}

#[tokio::test]
async fn use_testnet() -> Result<(), Box<dyn std::error::Error>> {
    let testnet = near_workspaces::testnet().await?;

    let account_id: AccountId = "test-ac-1719933221123-3.testnet".parse().unwrap();
    let secret_key: SecretKey = "ed25519:4ERg5chhrvzbqv4jUzbcSwejcEzzqaqxF5NL9RhPV3X9MF5pJjrSeWPMic8QcJaJz8mL7xHqgyQxZoHn6XJWstQe".parse().unwrap();

    let account = Account::from_secret_key(account_id, secret_key, &testnet);

    println!("account: {:#?}", account);
    // account: Account { id: AccountId("test-ac-1719933221123-3.testnet") }

    assert_eq!(account.id(), "test-ac-1719933221123-3.testnet");

    let hello_near_id: AccountId = "hello.near-examples.testnet".parse().unwrap();

    let current_greeting_on_testnet = account
        .call(&hello_near_id, "get_greeting")
        .view()
        .await?
        .json::<String>()
        .unwrap();

    println!(
        "current_greeting_on_testnet: {:#?}",
        current_greeting_on_testnet
    );
    // current_greeting_on_testnet: "defi is cool"

    let new_greeting = format!("{current_greeting_on_testnet} updated");
    let _ = account
        .call(&hello_near_id, "set_greeting")
        .args_json(json!({"greeting": new_greeting}))
        .max_gas()
        .transact()
        .await?;
    let updated_greeting_on_testnet = account
        .call(&hello_near_id, "get_greeting")
        .view()
        .await?
        .json::<String>()
        .unwrap();

    println!(
        "updated_greeting_on_testnet: {:#?}",
        updated_greeting_on_testnet
    );
    assert_eq!(updated_greeting_on_testnet, new_greeting);

    Ok(())
}

#[tokio::test]
async fn spoon_contract() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let testnet = near_workspaces::testnet_archival().await?;

    const BLOCK_HEIGHT: BlockHeight = 186705486;

    let hello_near_id: AccountId = "hello.near-examples.testnet".parse().unwrap();

    let sandbox_contract = sandbox
        .import_contract(&hello_near_id, &testnet)
        .block_height(BLOCK_HEIGHT)
        .initial_balance(NearToken::from_near(10))
        .transact()
        .await?;

    let greeting = sandbox_contract
        .call("get_greeting")
        .view()
        .await?
        .json::<String>()
        .unwrap();

    println!("greeting: {:#?}", greeting);
    // greeting: "Hello"

    // This is because "Hello" is a defailt greeting for Hello Near contract. When you spoon contract from archival node, you spoon only the contract, not the state.
    // That means if contract have to be initialized - you need to do that again calling initiliaze method
    assert_eq!(greeting, "Hello");

    Ok(())
}
