#![allow(non_snake_case)]
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

/// Main struct
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Fibo {
    /// 2 consecutive Fibonacci numbers and the position of the current one in Fibonacci sequence
    f1: u64,
    f2: u64,
    n: u64, 
}

/// Default implementation
impl Default for Fibo {
    fn default() -> Self {
        Fibo {
            f1: 0,
            f2: 1,
            n: 1,
        }
    }
}

/// Core logic
#[near_bindgen]
impl Fibo {
    pub fn log_number(&self, number: u64) {
        let log = format!("Got number: {}", number.to_string());
        env::log(log.as_bytes());
    }

    pub fn current_fibonacci_number(&self) -> u64 {
        if self.n == 1u64 {
            self.log_number(self.f1);
            return self.f1
        } 
        self.log_number(self.f2);
        return self.f2
    }

    pub fn next_fibonacci_number(&mut self) -> u64 {
        if self.n == 1 {
            self.n += 1;
            self.log_number(self.f1);
            return self.f1
        } else if self.n == 2 {
            self.n += 1;
            self.log_number(self.f2);
            return self.f2
        }
        let f3: u64 = self.f1 + self.f2;
        self.f1 = self.f2;
        self.f2 = f3;
        self.n += 1;
        self.log_number(self.f2);
        return self.f2
    }

    pub fn get_n(&self) -> u64 {
        return self.n
    }

    pub fn reset_sequence(&mut self) {
        env::log(b"Reset sequence");
        self.f1 = 0;
        self.f2 = 1;
        self.n = 1;
    }
}

/// Tests
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]

mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "bob.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "john.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn n_test() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Fibo::default();
        assert_eq!(1, contract.get_n());
        contract.next_fibonacci_number();
        assert_eq!(2, contract.get_n());
        contract.reset_sequence();
        assert_eq!(1, contract.get_n());
    }

    #[test]
    fn fibo_number_tests() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Fibo::default();
        assert_eq!(0, contract.current_fibonacci_number());
        assert_eq!(0, contract.next_fibonacci_number());
        assert_eq!(1, contract.next_fibonacci_number());
        assert_eq!(1, contract.next_fibonacci_number());
        assert_eq!(2, contract.next_fibonacci_number());
        assert_eq!(3, contract.next_fibonacci_number());
        assert_eq!(5, contract.next_fibonacci_number());
        assert_eq!(8, contract.next_fibonacci_number());
        assert_eq!(13, contract.next_fibonacci_number());
    }

    #[test]
    fn reset_test() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Fibo::default();
        contract.reset_sequence();
        assert_eq!(0, contract.f1);
        assert_eq!(1, contract.f2);
        assert_eq!(1, contract.n);
    }
}