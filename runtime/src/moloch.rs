#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{ decl_module, dispatch::DispatchResult, debug };
use frame_system::{ self as system, ensure_signed };

#[derive(PartialEq, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Proposal<Hash, Balance> { 
    proposer: Hash, 
    shares_amount: Balance, 
}

type SubmittedProposal<T> = SubmittedProposal<<T as system::Trait>::Hash, <T as balances::Trait>::Balance>;

decl_event! {
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        create_proposal(AccountId),
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as MolochStorage {
          // Declare storage and getter functions here
  }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Declare public functions here
        fn deposit_event() = default;

        fn create_proposal(origin, proposer: Hash, hash: T::Hash, balance: T::Balance) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let new_proposal = Proposal {
                proposer,
                shares_amount,
            };

            <SubmittedProposal<T>>::insert(&sender, new_proposal);
            Self::deposit_event(RawEvent::Created(sender, new_proposal));
            
            Ok(())
        }
    }
}