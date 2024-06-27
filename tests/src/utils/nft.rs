use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::{AccountId, NearToken};
use serde_json::json;

pub trait NftExt {
    async fn nft_transfer(
        &self,
        collection: &AccountId,
        receiver_id: &AccountId,
        token_id: TokenId,
        memo: Option<String>,
    ) -> anyhow::Result<()>;

    async fn nft_transfer_call(
        &self,
        collection: &AccountId,
        receiver_id: &AccountId,
        token_id: TokenId,
        memo: Option<String>,
        msg: String,
    ) -> anyhow::Result<bool>;

    async fn nft_token(&self, token_id: &TokenId) -> anyhow::Result<Option<Token>>;
}

impl NftExt for near_workspaces::Account {
    async fn nft_transfer(
        &self,
        collection: &AccountId,
        receiver_id: &AccountId,
        token_id: TokenId,
        memo: Option<String>,
    ) -> anyhow::Result<()> {
        self.call(collection, "nft_transfer")
            .args_json(json!({
                "receiver_id": receiver_id,
                "token_id": token_id,
                "memo": memo,
            }))
            .deposit(NearToken::from_yoctonear(1))
            .max_gas()
            .transact()
            .await?
            .into_result()?;
        Ok(())
    }

    async fn nft_transfer_call(
        &self,
        collection: &AccountId,
        receiver_id: &AccountId,
        token_id: TokenId,
        memo: Option<String>,
        msg: String,
    ) -> anyhow::Result<bool> {
        self.call(collection, "nft_transfer_call")
            .args_json(json!({
                "receiver_id": receiver_id,
                "token_id": token_id,
                "memo": memo,
                "msg": msg,
            }))
            .deposit(NearToken::from_yoctonear(1))
            .max_gas()
            .transact()
            .await?
            .into_result()?
            .json()
            .map_err(Into::into)
    }

    async fn nft_token(&self, token_id: &TokenId) -> anyhow::Result<Option<Token>> {
        self.view(self.id(), "nft_token")
            .args_json(json!({
                "token_id": token_id,
            }))
            .await?
            .json()
            .map_err(Into::into)
    }
}
