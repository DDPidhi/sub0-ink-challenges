#![cfg_attr(not(feature = "std"), no_std, no_main)]

// # ✒️ Challenge 2: Implement voter registration, proposal management, and voting in your Dao.
//
// - **Difficulty**: Mid
// - **Submission Criteria:** ink! contract must
//     - Use a storage-optimized data-structure `Mapping` or `StorageVec`
//     - Store registered members, member votes, and proposals to vote on.
//     - Implement methods to register and de-register members.
//     - Implement methods to create proposals and a method to vote on proposals.
//     - Unit tests for adding members, votes, and proposals.
// - **Submission Guidelines:**
//     - Verify with R0GUE DevRel, and post on X.
// - **Prize:** sub0 merch

#[ink::contract]
mod dao {
    use ink::{
        prelude::string::String,
        prelude::vec::Vec,
        storage::{Mapping, Lazy},
    };
    use minidao_common::*;

    #[derive(Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout)
    )]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct BasicProposal {
        pub vote_count: u32,
    }


    #[derive(Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout)
    )]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Member {
        pub vote_count: u32,
        pub name: String,
    }

    #[ink(storage)]
    pub struct Dao {
        name: String,
        registered_members: Mapping<AccountId, Member>,
        proposals: Lazy<Vec<BasicProposal>>,
    }

    impl Dao {
        // Constructor that initializes the values for the contract.
        #[ink(constructor)]
        pub fn new(initial_name: String) -> Self {
            Self {
                name: initial_name,
                registered_members: Mapping::default(),
                proposals: Lazy::default(),
            }
        }

        // Constructor that initializes the default values for the contract.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn get_name(&self) -> String {
            self.name.clone()
        }

        #[ink(message)]
        pub fn register_voter(&mut self, voter_id: AccountId, voter_name: String) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterAlreadyRegistered` if the voter is registered
            // - Success: Register a new `voter` to the Dao

            if self.registered_members.contains(&voter_id) {
                return Err(DaoError::VoterAlreadyRegistered);
            }

            let member = Member {
                vote_count: 0,
                name: voter_name,
            };

            self.registered_members.insert(&voter_id, &member);

            Ok(())
        }

        #[ink(message)]
        pub fn deregister_voter(&mut self, voter_id: AccountId) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Deregister a new `voter` from the Dao

            if !self.registered_members.contains(&voter_id) {
                return Err(DaoError::VoterNotRegistered);
            }

            self.registered_members.remove(&voter_id);

            Ok(())
        }

        #[ink(message)]
        pub fn has_voter(&self, voter_id: AccountId) -> bool {
            // - Success: Return if the voter is registered.
            self.registered_members.contains(&voter_id)
        }

        #[ink(message)]
        pub fn create_proposal(&mut self, voter_id: AccountId) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Create a new proposal that stores `votes` from `voters`

            if !self.registered_members.contains(&voter_id) {
                return Err(DaoError::VoterNotRegistered);
            }

            let proposal = BasicProposal {
                vote_count: 0,
            };

            let mut proposals = self.proposals.get_or_default();
            proposals.push(proposal);

            self.proposals.set(&proposals);

            Ok(())
        }

        #[ink(message)]
        pub fn remove_proposal(&mut self, proposal_id: u32, voter_id: AccountId) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Error: Throw error `DaoError::ProposalDoesNotExist` if the proposal is not created
            // - Success: Create a new proposal that stores `votes` from `voters`
            //              -- I am assuming that this comment is incorrect and in case of success
            //                  we need to remove the specified proposal

            if !self.registered_members.contains(&voter_id) {
                return Err(DaoError::VoterNotRegistered);
            }

            if self.proposals.get_or_default().len() <= proposal_id.try_into().unwrap() {
                return Err(DaoError::ProposalDoesNotExist);
            }

            let mut proposals = self.proposals.get_or_default();
            proposals.remove(proposal_id.try_into().unwrap());

            Ok(())
        }

        #[ink(message)]
        pub fn get_proposal(&self, proposal_id: u32) -> Result<Option<BasicProposal>, DaoError> {
            // - Success: Returns the proposal detail

            let proposals = self.proposals.get_or_default();
            if proposals.len() <= proposal_id.try_into().unwrap() {
                return Err(DaoError::ProposalDoesNotExist);
            }

            Ok(proposals.get(proposal_id as usize).cloned())
        }

        #[ink(message)]
        pub fn vote(&mut self, proposal_id: u32, voter_id: AccountId) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Error: Throw error `Error::ProposalDoesNotExist` if the proposal is not created
            // - Success: Vote on the proposal

            if !self.registered_members.contains(&voter_id) {
                return Err(DaoError::VoterNotRegistered);
            }

            let mut proposals = self.proposals.get_or_default();
            if proposals.len() <= proposal_id.try_into().unwrap() {
                return Err(DaoError::ProposalDoesNotExist);
            }

            let mut member = self.registered_members.get(&voter_id).unwrap();
            member.vote_count += 1;
            self.registered_members.insert(&voter_id, &member);

            let proposal = proposals.get_mut(proposal_id as usize).unwrap();
            proposal.vote_count += 1;

            Ok(())
        }

        #[ink(message)]
        pub fn vote_count(&self, voter_id: AccountId) -> Result<u32, DaoError> {
            // - Returns the number of `votes` a Dao `voter` voted

            if !self.registered_members.contains(&voter_id) {
                return Err(DaoError::VoterNotRegistered);
            }

            Ok(self.registered_members.get(&voter_id).unwrap().vote_count)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::dao::Dao;

        #[ink::test]
        fn test_voter_registration() {
            let mut dao = Dao::new(String::from("peppysheppy-dev"));
            let voter_id: AccountId = AccountId::from([0x01; 32]);
            let voter_name = String::from("PeppySheppy");

            assert!(dao.register_voter(voter_id, voter_name.clone()).is_ok());

            let result = dao.register_voter(voter_id, voter_name);
            assert!(matches!(result, Err(DaoError::VoterAlreadyRegistered)), "Expected VoterAlreadyRegistered error");

            assert_eq!(dao.has_voter(voter_id), true);

            assert!(dao.deregister_voter(voter_id).is_ok());

            assert_eq!(dao.has_voter(voter_id), false);

            let result = dao.deregister_voter(voter_id);

            assert!(matches!(result, Err(DaoError::VoterNotRegistered)), "Expected VoterNotRegistered error");
        }

        #[ink::test]
        fn test_proposal_management() {
            let mut dao = Dao::new(String::from("peppysheppy-dev"));
            let voter_id: AccountId = AccountId::from([0x01; 32]);
            let voter_name = String::from("PeppySheppy");

            let result = dao.create_proposal(voter_id);
            assert!(matches!(result, Err(DaoError::VoterNotRegistered)), "Expected VoterNotRegistered error");

            let _ = dao.register_voter(voter_id, voter_name.clone()).is_ok();

            assert!(dao.create_proposal(voter_id).is_ok());

            let result = dao.remove_proposal(2, voter_id);
            assert!(matches!(result, Err(DaoError::ProposalDoesNotExist)), "Expected ProposalDoesNotExist error");

            assert!(dao.remove_proposal(0, voter_id).is_ok());
        }

        #[ink::test]
        fn test_vote() {
            let mut dao = Dao::new(String::from("peppysheppy-dev"));
            let voter_id: AccountId = AccountId::from([0x01; 32]);
            let voter_name = String::from("PeppySheppy");
            let _ = dao.register_voter(voter_id, voter_name.clone()).is_ok();
            let _ = dao.create_proposal(voter_id);

            assert!(dao.vote(0, voter_id).is_ok());

            assert!(matches!(dao.vote_count(voter_id), Ok(1)), "Expected vote count 1");
        }
    }
}
