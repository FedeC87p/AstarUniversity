#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod dao {
    use ink::storage::Mapping;
    use openbrush::contracts::traits::psp22::*;
    use scale::{
        Decode,
        Encode,
    };

    // Consts.
    const ONE_MINUTE: u64 = 60;

    // Types.
    type ProposalId = u64;

    // Enums.
    #[derive(Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq, scale_info::TypeInfo))]
    pub enum VoteType {
        Positive,
        Negative
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum GovernorError {
        AmountShouldNotBeZero,
        DurationError,
        QuorumNotReached
    }

    // Structs.
    #[derive(Encode, Decode)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink::storage::traits::StorageLayout
        )
    )]
    pub struct Proposal {
        to: AccountId,
        amount: Balance,
        vote_start: u64,
        vote_end: u64,
        executed: bool
        vote: ProposalVote,
        histories: Mapping<AccountId, bool>
    }

    #[derive(Encode, Decode, Default)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink::storage::traits::StorageLayout
        )
    )]
    pub struct ProposalVote {
        positive: Balance,
        negative: Balance
    }

    #[ink(storage)]
    pub struct Governor {
        proposals: Mapping<u64, Proposal>,
        proposal_counter: ProposalId
    }

    impl Governor {
        #[ink(constructor, payable)]
        pub fn new(governance_token: AccountId, quorum: u8) -> Self {
            Self {
                proposals: Mapping::new()
                proposal_counter: 0
            }
        }

        #[ink(message)]
        pub fn propose(
            &mut self,
            to: AccountId,
            amount: Balance,
            duration: u64,
        ) -> Result<(), GovernorError> {
            if (amount <= 0) 
                return Err(GovernorError::AmountShouldNotBeZero)
            if (duration <= 0) 
                return Err(GovernorError::DurationError)

            let proposal = Proposal {
                to,
                amount,
                vote_start: self.env().block_timestamp(),
                vote_end: self.env().block_timestamp() + (ONE_MINUTE * duration),
                executed: false,
                histories: Mapping::new()
            };
            self.proposals.insert(self.next_proposal_id, &proposal);

            self.next_proposal_id++;

            Ok(())
        }

        #[ink(message)]
        pub fn vote(
            &mut self,
            proposal_id: ProposalId,
            vote: VoteType,
        ) -> Result<(), GovernorError> {
            if (amount < 0) 
                return Err(GovernorError::ProposalNotFound)

            let proposal = self
                .proposals
                .get(&proposal_id)
                .ok_or(GovernorError::ProposalNotFound)?;



        }

        #[ink(message)]
        pub fn execute(&mut self, proposal_id: ProposalId) -> Result<(), GovernorError> {
            unimplemented!()
        }

        // used for test
        #[ink(message)]
        pub fn now(&self) -> u64 {
            self.env().block_timestamp()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn create_contract(initial_balance: Balance) -> Governor {
            let accounts = default_accounts();
            set_sender(accounts.alice);
            set_balance(contract_id(), initial_balance);
            Governor::new(AccountId::from([0x01; 32]), 50)
        }

        fn contract_id() -> AccountId {
            ink::env::test::callee::<ink::env::DefaultEnvironment>()
        }

        fn default_accounts(
        ) -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        fn set_sender(sender: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(sender);
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(
                account_id, balance,
            )
        }

        #[ink::test]
        fn propose_works() {
            let accounts = default_accounts();
            let mut governor = create_contract(1000);
            assert_eq!(
                governor.propose(accounts.django, 0, 1),
                Err(GovernorError::AmountShouldNotBeZero)
            );
            assert_eq!(
                governor.propose(accounts.django, 100, 0),
                Err(GovernorError::DurationError)
            );
            let result = governor.propose(accounts.django, 100, 1);
            assert_eq!(result, Ok(()));
            let proposal = governor.get_proposal(0).unwrap();
            let now = governor.now();
            assert_eq!(
                proposal,
                Proposal {
                    to: accounts.django,
                    amount: 100,
                    vote_start: 0,
                    vote_end: now + 1 * ONE_MINUTE,
                    executed: false,
                }
            );
            assert_eq!(governor.next_proposal_id(), 1);
        }

        #[ink::test]
        fn quorum_not_reached() {
            let mut governor = create_contract(1000);
            let result = governor.propose(AccountId::from([0x02; 32]), 100, 1);
            assert_eq!(result, Ok(()));
            let execute = governor.execute(0);
            assert_eq!(execute, Err(GovernorError::QuorumNotReached));
        }
    }
}
