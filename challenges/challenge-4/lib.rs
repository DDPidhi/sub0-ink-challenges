#![cfg_attr(not(feature = "std"), no_std, no_main)]

// # ✒️ Challenge 4: Support creating cross-chain proposals to the Super DAO

// - **Difficulty**: Advanced
// - **Submission Criteria:** ink! contract must
//     - Support creating cross-chain proposals to the Super DAO (XCM)
//     - A deployed contract on Pop Network Testnet
//     - Have a cross-chain proposal successfully executed
// - **Submission Guidelines:**
//     - Verify with R0GUE DevRel, and post on X.
// - **Prize:** Sub0 merch

#[ink::contract]
mod dao {
    use ink::{
        contract_ref,
        prelude::string::String,
        storage::StorageVec,
        xcm::prelude::*,
        prelude::vec::Vec,
    };
    use minidao_common::*;
    use superdao_traits::{Call, ChainCall, Error, SuperDao, Vote};

    #[ink(storage)]
    pub struct Dao {
        superdao: contract_ref!(SuperDao),
        voters: StorageVec<AccountId>,
        name: String,
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
            };
            assert!(instance.superdao.register_member().is_ok(), "Superdao registration failed!");
            instance
        }

        #[ink(message)]
        pub fn get_name(&self) -> String {
            // - Returns the name of the Dao
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
        pub fn deregister_voter(&mut self, voter: crate::dao::AccountId) -> Result<(), DaoError> {
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
        pub fn create_superdao_cross_chain_proposal(
            &mut self,
            voter: AccountId,
            encoded_extrinsic: Vec<u8>,
            fee_max: Balance,
            ref_time: u64,
            proof_size: u64,
        ) -> Result<Result<u32, Error>, DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Create a SuperDao proposal to execute a cross-chain message.

            if !self.has_voter(voter) {
                return Err(DaoError::VoterNotRegistered);
            }

            let asset: Asset = (Location::parent(), fee_max).into();
            let ah = Junctions::from([Parachain(1000)]);
            let dest: Location = Location { parents: 1, interior: ah};

            let message: Xcm<()> = Xcm::builder()
                .withdraw_asset(asset.clone().into())
                .buy_execution(asset.clone(), Unlimited)
                .transact(
                    OriginKind::SovereignAccount,
                    Weight::from_parts(ref_time, proof_size),
                    encoded_extrinsic.into(),
                )
                .build();

            let call = Call::Chain(ChainCall::new(&dest, &message));

            Ok(self.superdao.create_proposal(call))
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

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::dao::Dao;

        #[ink::test]
        fn test_vote_superdao_cross_chain_proposal() {
            todo!("Challenge 4");
        }
    }
}
