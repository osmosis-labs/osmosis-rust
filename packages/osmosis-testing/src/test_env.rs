use crate::{GetAllBalances, InitAccount, InitTestEnv};
use cosmwasm_std::Coin;
use std::ffi::CString;

pub struct TestEnv {
    id: u64,
}

impl TestEnv {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            id: unsafe { InitTestEnv() },
        }
    }

    /// This function initialize account with initial balance of any coins.
    /// It returns bech32 address of the account initialized.
    pub fn init_account(&self, coins: &[Coin]) -> String {
        let coins_json = serde_json::to_string(&coins).unwrap();
        let coins_json_c = &CString::new(coins_json).unwrap();

        unsafe {
            let addr = InitAccount(self.id, coins_json_c.into());
            CString::from_raw(addr)
        }
        .to_str()
        .expect("invalid utf8")
        .to_string()
    }

    /// Get all balances of an account
    pub fn get_all_balances(&self, bech32_addr: &str) -> Vec<Coin> {
        let addr_c = &CString::new(bech32_addr).unwrap();
        let bal = unsafe {
            let bal = GetAllBalances(self.id, addr_c.into());
            CString::from_raw(bal)
        }
        .to_str()
        .expect("invalid utf8")
        .to_string();

        serde_json::from_str(&bal).expect("invalid Vec<Coin> json")
    }
}

#[cfg(test)]
mod tests {
    use crate::test_env::TestEnv;
    use cosmwasm_std::{coins, Coin};

    #[test]
    fn test_init_account() {
        let env = TestEnv::new();

        // alice
        let alice_balance = coins(100_000_000_000, "uosmo");
        let alice = env.init_account(&alice_balance);
        assert_eq!(env.get_all_balances(&alice), alice_balance);

        // bob
        let bob_balance = [
            Coin::new(100_000_000_000, "uatom"),
            Coin::new(999_999_999_999, "uion"),
        ];
        let bob = env.init_account(&bob_balance);
        assert_eq!(env.get_all_balances(&bob), bob_balance);
    }
}
