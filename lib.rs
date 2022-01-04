#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {
    use ink_prelude::string::String;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Incrementer {
        /// Stores a single `bool` value on the storage.
        value: i32,
        optional_value: Option<i32>,
        account_value_map: ink_storage::collections::HashMap<AccountId, i32>,
    }

    #[ink(event)]
    pub struct IncResult {
        #[ink(topic)]
        caller: AccountId,
        #[ink(topic)]
        current_value: Result<i32, String>,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self {
                value: init_value,
                optional_value: Some(init_value),
                account_value_map: Default::default(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        #[ink(message)]
        pub fn get_optional(&self) -> Option<i32> {
            self.optional_value
        }

        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.value += by;
            self.optional_value = Some(self.optional_value.unwrap() + by);
        }

        #[ink(message)]
        pub fn inc_and_return_value(&mut self, by: i32) -> i32 {
            self.inc(by);
            self.get()
        }

        #[ink(message)]
        pub fn inc_and_emit_event(&mut self, by: i32) {
            self.inc(by);
            self.env().emit_event(IncResult {
                caller: self.env().caller(),
                current_value: Ok(self.get()),
            });
        }

        #[ink(message)]
        pub fn inc_and_emit_event_and_fail(&mut self, by: i32) -> Result<i32, String> {
            self.inc(by);

            let err_msg: String = "Handmade error".into();
            self.env().emit_event(IncResult {
                caller: self.env().caller(),
                current_value: Err(err_msg.clone()),
            });
            return Err(err_msg);
        }

        #[ink(message)]
        pub fn divided_by_zero_and_fail(&mut self, manual_zero: i32) -> i32 {
            3 / manual_zero
        }

        #[ink(message)]
        pub fn get_my_value_or_zero(&self) -> i32 {
            let caller = self.env().caller();
            self.get_account_value_or_zero(caller)
        }

        #[ink(message)]
        pub fn incr_my_value(&mut self, by: i32) {
            // ACTION: Get the `caller` of this function.
            // ACTION: Get `my_value` that belongs to `caller` by using `my_value_or_zero`.
            // ACTION: Insert the incremented `value` back into the mapping.
            let caller = self.env().caller();
            self.account_value_map
                .entry(caller)
                .and_modify(|value| *value += by)
                .or_insert(by);
        }

        fn get_account_value_or_zero(&self, account_id: AccountId) -> i32 {
            *self.account_value_map.get(&account_id).unwrap_or(&0)
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let incrementer = Incrementer::default();
            assert_eq!(incrementer.get(), 0);
            assert_eq!(incrementer.get_optional(), Some(0));
        }

        #[ink::test]
        fn constructor_works() {
            let incrementer = Incrementer::new(0);
            assert_eq!(incrementer.get(), 0);
            assert_eq!(incrementer.get_optional(), Some(0));
        }

        #[ink::test]
        fn get_be_equal_to_get_optional() {
            let mut incrementer = Incrementer::new(0);
            assert_eq!(incrementer.get(), 0);
            assert_eq!(incrementer.get_optional().unwrap(), incrementer.get());

            incrementer.inc(2);
            assert_eq!(incrementer.get_optional().unwrap(), incrementer.get());
        }

        #[ink::test]
        fn it_works() {
            let mut contract = Incrementer::new(42);
            assert_eq!(contract.get(), 42);
            contract.inc(5);
            assert_eq!(contract.get(), 47);
        }

        #[ink::test]
        fn my_value_works() {
            let mut contract = Incrementer::new(11);
            assert_eq!(contract.get(), 11);
            assert_eq!(contract.get_my_value_or_zero(), 0);
            contract.incr_my_value(2);
            assert_eq!(contract.get_my_value_or_zero(), 2);
        }
    }
}
