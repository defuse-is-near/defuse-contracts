use near_workspaces::types::NearToken;
use near_workspaces::{Account, AccountId, Contract, Worker};
use serde_json::json;

// TODO: use near_workspaces::compile_project("./some/path").await?;
const ACCOUNT_WASM: &[u8] = include_bytes!("../../../res/defuse-account-contract.wasm");
const CONTROLLER_WASM: &[u8] = include_bytes!("../../../res/defuse-controller-contract.wasm");
const INTENT_WASM: &[u8] = include_bytes!("../../../res/defuse-intent-contract.wasm");
const TOKEN_WASM: &[u8] = include_bytes!("../../../res/fungible-token.wasm");

const TOTAL_SUPPLY: u128 = 1_000_000_000;

pub struct Sandbox {
    worker: Worker<near_workspaces::network::Sandbox>,
    root_account: Account,
}

impl Sandbox {
    pub async fn new() -> anyhow::Result<Self> {
        let worker = near_workspaces::sandbox().await?;
        let root_account = worker.root_account()?;

        Ok(Self {
            worker,
            root_account,
        })
    }

    pub async fn create_subaccount(
        &self,
        name: &str,
        balance: NearToken,
    ) -> anyhow::Result<Account> {
        self.root_account
            .create_subaccount(name)
            .initial_balance(balance)
            .transact()
            .await
            .map(|result| result.result)
            .map_err(Into::into)
    }

    pub async fn create_account(&self, name: &str) -> Account {
        self.create_subaccount(name, NearToken::from_near(100))
            .await
            .unwrap()
    }

    pub async fn balance(&self, account_id: &AccountId) -> u128 {
        self.worker
            .view_account(account_id)
            .await
            .unwrap()
            .balance
            .as_yoctonear()
    }

    pub async fn deploy_account_contract(&self) -> Contract {
        let contract = self.deploy_contract("account", ACCOUNT_WASM).await;
        let result = contract
            .call("new")
            .args_json(json!({
                "owner_id": "controller.test.near",
                "mpc_contract_id": "mpc.test.net"
            }))
            .max_gas()
            .transact()
            .await
            .unwrap();
        assert!(result.is_success(), "{result:#?}");

        contract
    }

    pub async fn deploy_controller_contract(
        &self,
        dao: &AccountId,
        mpc_contract_id: &AccountId,
    ) -> Contract {
        let contract = self.deploy_contract("controller", CONTROLLER_WASM).await;
        let result = contract
            .call("new")
            .args_json(json!({
                // TODO: add support for DAO in Sandbox, not TLA
                "dao": dao,
                "mpc_contract_id": mpc_contract_id,
            }))
            .max_gas()
            .transact()
            .await
            .unwrap();
        assert!(result.is_success(), "{result:#?}");

        contract
    }

    pub async fn deploy_intent_contract(&self) -> Contract {
        let contract = self.deploy_contract("intent", INTENT_WASM).await;
        let result = contract
            .call("new")
            .args_json(json!({
                "owner_id": contract.id()
            }))
            .max_gas()
            .transact()
            .await
            .unwrap();
        assert!(result.is_success(), "{result:#?}");

        contract
    }

    pub async fn deploy_token(&self, token: &str) -> Contract {
        let contract = self.deploy_contract(token, TOKEN_WASM).await;
        let result = contract
            .call("new")
            .args_json(json!({
                "owner_id": contract.id(),
                "total_supply": TOTAL_SUPPLY.to_string(),
                "metadata": {
                    "spec": "ft-1.0.0",
                    "name": format!("Token {}", &token),
                    "symbol": "TKN",
                    "decimals": 18
                }
            }))
            .max_gas()
            .transact()
            .await
            .unwrap();
        assert!(result.is_success(), "{result:#?}");

        contract
    }

    async fn deploy_contract(&self, account_id: &str, wasm: &[u8]) -> Contract {
        let contract_id = self
            .create_subaccount(account_id, NearToken::from_near(10))
            .await
            .unwrap();
        let result = contract_id.deploy(wasm).await.unwrap();
        assert!(result.is_success());

        result.result
    }
}
