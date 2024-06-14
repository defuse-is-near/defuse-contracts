use defuse_contracts::controller::ControllerContract;
use near_sdk::{near, AccountId, PanicOnDefault};

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct ControllerContractImpl {
    owner_id: AccountId,
}

#[near]
impl ControllerContract for ControllerContractImpl {}

#[near]
impl ControllerContractImpl {
    #[init]
    #[must_use]
    #[allow(clippy::use_self)]
    pub const fn new(owner_id: AccountId) -> Self {
        Self { owner_id }
    }
}
