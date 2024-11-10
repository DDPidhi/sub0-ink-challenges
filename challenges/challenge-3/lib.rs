#![cfg_attr(not(feature = "std"), no_std, no_main)]

// # ✒️ Challenge 3: Connect your DAO to the Super DAO with registration and voting
//
// - **Difficulty**: Mid
// - **Submission Criteria:** ink! contract must
//     - Import the Super DAO trait - Already provided
//     - Store Super DAO contract address. - Already provided
//     - Register contract as member of Super DAO - using trait-based contract calling.
//     - Vote on proposals in the Super DAO - using trait-based contract calling.
//     - Create proposals to call another contract - using trait-based contract calling.
//     - E2E test for cross-contract call.
// - **Submission Guidelines:**
//     - Verify with R0GUE DevRel, and post on X.
// - **Prize:** Sub0 Merch & ink! sports towel

#[ink::contract]
mod dao {
    use ink::{contract_ref, prelude::{string::String, vec}, selector_bytes, storage::StorageVec};
    use ink::codegen::Env;
    use minidao_common::*;
    use superdao_traits::{Call, ContractCall, Error, SuperDao, Vote};

    #[ink(storage)]
    pub struct Dao {
        superdao: contract_ref!(SuperDao),
        voters: StorageVec<AccountId>,
        name: String,
        value: u8,
    }

    #[ink(event)]
    pub struct DaoMessage {
        value: u8,
    }

    impl Dao {
        // Constructor that initializes the values for the contract.
        #[ink(constructor)]
        pub fn new(name: String, superdao: AccountId) -> Self {
            // Register your Dao as a member of the Superdao.
            let mut instance = Self {
                name,
                superdao: superdao.into(),
                voters: StorageVec::new(),
                value: 0,
            };
            assert!(instance.superdao.register_member().is_ok(), "Unable to register SuperDao");
            instance
        }

        // #[ink(message)]
        // pub fn initialize_superdao(&mut self) {
        //     let _ = self.superdao.register_member();
        // }

        #[ink(message)]
        pub fn get_name(&self) -> String {
            self.name.clone()
        }

        #[ink(message)]
        pub fn register_voter(&mut self, voter: AccountId) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterAlreadyRegistered` if the voter is registered
            // - Success: Register a new `voter` to the Dao
            if self.has_voter(voter) {
                return Err(DaoError::VoterAlreadyRegistered);
            }
            self.voters.push(&voter);
            Ok(())
        }

        #[ink(message)]
        pub fn deregister_voter(&mut self, voter: AccountId) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Deregister a new `voter` from the Dao

            if !self.has_voter(voter) {
                return Err(DaoError::VoterNotRegistered);
            }
            let mut new_voters = StorageVec::new();
            while let Some(stored_voter) = self.voters.pop() {
                if stored_voter != voter {
                    new_voters.push(&stored_voter)
                }
            }
            self.voters = new_voters;
            Ok(())
        }

        #[ink(message)]
        pub fn has_voter(&self, voter: AccountId) -> bool {
            let mut result = false;
            for i in 0..self.voters.len() {
                if let Some(v) = self.voters.get(i) {
                    if v == voter {
                        result = true;
                        break;
                    }
                }
            }
            result
        }

        #[ink(message)]
        pub fn create_contract_call_proposal(&mut self, voter: AccountId, call: Call) -> Result<Result<u32, Error>, DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Create a SuperDao proposal to call a contract method.

            if !self.has_voter(voter) {
                return Err(DaoError::VoterNotRegistered);
            }

            // let call = Call::Contract(ContractCall {
            //     callee: self.env().account_id(),
            //     selector: selector_bytes!("update_value"),
            //     input: vec![],
            //     transferred_value: 0,
            //     ref_time_limit: 0,
            //     allow_reentry: false,
            // });

            Ok(self.superdao.create_proposal(call))
        }

        #[ink(message)]
        pub fn update_value(&mut self) -> Result<(), DaoError> {
            self.value = self.value.saturating_add(10);
            Ok(())
        }

        #[ink(message)]
        pub fn get_value(&mut self) -> u8 {
            self.value
        }

        #[ink(message)]
        pub fn vote_proposal(&mut self, proposal_id: u32, vote: bool, voter: AccountId) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Vote a SuperDao proposal.

            if !self.has_voter(voter) {
                return Err(DaoError::VoterNotRegistered);
            }

            let vote = if vote { Vote::Aye } else { Vote::Nay };

            assert!(self.superdao.vote(proposal_id, vote).is_ok(), "Unable to vote proposal");

            Ok(())
        }
    }

}
