#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_storage::collections::{Vec, HashMap, Stash, Bitvec};


#[ink::contract]
mod incrementer {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Incrementer {
        /// Stores a single `bool` value on the storage.
        bool_value: bool,

        // store a number
        number: u32,

        // store a number using lazy
        lazy_number: ink_storage::Lazy<u32>,


        // store some AccountId
        account_id: AccountId,

        //store some Balance
        balance: Balance,
    }

    impl Incrementer {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool, init_number: u32, init_lazy_number: u32, init_account: AccountId, init_balance: Balance) -> Self {
            Self {
                bool_value: init_value,
                number: init_number,
                lazy_number: ink_storage::Lazy::<u32>::new(init_lazy_number), 
                account_id:  init_account,
                balance: init_balance,
             }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default(), Default::default(), Default::default(), Default::default(), Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.bool_value = !self.bool_value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get_bool(&self) -> bool {
            self.bool_value
        }

        /// Simply returns the current value of our `number`.
        #[ink(message)]
        pub fn get_number(&self) -> u32 {
            self.number
        }

        /// Simply set the current value of our `number`.
        #[ink(message)]
        pub fn set_number(&mut self, new_value: u32) {
            self.number = new_value;
        }

        /// Simply increment the current value of our `number`.
        #[ink(message)]
        pub fn inc(&mut self, by: u32) {
            self.number += by;
        }

        /// Simply set the current value of our `number`.
        #[ink(message)]
        pub fn get_number_lazy (&mut self) -> u32 {
            let x = ink_storage::Lazy::<u32>::get(&mut self.lazy_number);
            x.clone()
        }

        /// Simply set the current value of our `number`.
        #[ink(message)]
        pub fn set_number_lazy (&mut self, new_value: u32) {
            ink_storage::Lazy::<u32>::set(&mut self.lazy_number, new_value);
        }

        /// Simply increment the current value of our `number`.
        #[ink(message)]
        pub fn inc_lazy(&mut self, by: u32) {
            let my_lazy_number = &mut self.lazy_number;
            let cur = ink_storage::Lazy::<u32>::get(my_lazy_number);
            ink_storage::Lazy::<u32>::set(my_lazy_number, cur + by);
        }

        /// Simply returns the current value of our `account`.
        #[ink(message)]
        pub fn get_account(&self) -> AccountId {
            self.account_id
        }

        /// Simply returns the current value of our `balance`.
        #[ink(message)]
        pub fn get_balance(&self) -> Balance {
            self.balance
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

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let incrementer = Incrementer::default();
            assert_eq!(incrementer.get_bool(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut incrementer = Incrementer::new(false, 4, Default::default(), Default::default(), Default::default());
            assert_eq!(incrementer.get_bool(), false);
            incrementer.flip();
            assert_eq!(incrementer.get_bool(), true);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn number_test() {
            let mut incrementer = Incrementer::new(false, 4, Default::default(), Default::default(), Default::default());
            assert_eq!(incrementer.get_number(), 4);
            incrementer.set_number(3);
            assert_eq!(incrementer.get_number(), 3);
            incrementer.inc(4);
            assert_eq!(incrementer.get_number(), 7);
            incrementer.inc(1);
            assert_eq!(incrementer.get_number(), 8);
            incrementer.inc(2);
            assert_eq!(incrementer.get_number(), 10);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn lazy_number_test() {
            let mut incrementer = Incrementer::new(false, 4, 4, Default::default(), Default::default());
            assert_eq!(incrementer.get_number_lazy(), 4);
            incrementer.set_number_lazy(3);
            assert_eq!(incrementer.get_number_lazy(), 3);
            incrementer.inc_lazy(4);
            assert_eq!(incrementer.get_number_lazy(), 7);
            incrementer.inc_lazy(1);
            assert_eq!(incrementer.get_number_lazy(), 8);
            incrementer.inc_lazy(2);
            assert_eq!(incrementer.get_number_lazy(), 10);
        }

    }
}
