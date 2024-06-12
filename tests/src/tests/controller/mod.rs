mod env;

use env::Env;
use near_sdk::AccountId;

#[tokio::test]
async fn test_deploy_controller() {
    Env::create().await;
}

#[tokio::test]
async fn test_deploy_account_shard() {
    let env = Env::create().await;
    let account_shard = env.deploy_account_shard("account-shard").await;

    let account_shard_owner: AccountId = env
        .user
        .view(&account_shard, "owner_get")
        .await
        .unwrap()
        .json()
        .unwrap();
    assert_eq!(account_shard_owner, *env.controller.id());

    let account_shard_mpc_contract: AccountId = env
        .user
        .view(&account_shard, "get_mpc_contract")
        .await
        .unwrap()
        .json()
        .unwrap();
    assert_eq!(account_shard_mpc_contract, *env.mpc_contract.id());

    println!("upgrading...");

    let account_shard_upgraded = env.deploy_account_shard("account-shard").await;
    assert_eq!(
        account_shard_upgraded, account_shard,
        "redeployed contract must have the same account ID"
    );

    let account_shard_owner: AccountId = env
        .user
        .view(&account_shard, "owner_get")
        .await
        .unwrap()
        .json()
        .unwrap();
    assert_eq!(account_shard_owner, *env.controller.id());

    let account_shard_mpc_contract: AccountId = env
        .user
        .view(&account_shard, "get_mpc_contract")
        .await
        .unwrap()
        .json()
        .unwrap();
    assert_eq!(account_shard_mpc_contract, *env.mpc_contract.id());
}
