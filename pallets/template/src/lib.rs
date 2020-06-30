#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_module, decl_storage, decl_event, decl_error, ensure, StorageMap,
	traits::{
		Get
	},
	codec::{Encode, Decode}
};
use frame_system::{self as system, ensure_signed};
use sp_std::vec::Vec;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	type StartingPeriod: Get<Self::BlockNumber>;

	type VotingPeriod: Get<Self::BlockNumber>;
}

type ProposalIndex = u32;
type TokenIndex = u32;

#[derive(Encode, Decode, Clone, Default, PartialEq, Eq)]
pub struct ProposalInfo<AccountId, BlockNumber> {
	proposer: AccountId, 
	starting_period: BlockNumber,
	yes_votes: u32,
	no_votes: u32,
	processed: bool,
	did_pass: bool,
	aborted: bool,
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct TokenCreation {
	tribute_token: u32,
	payment_token: u32,
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Member {
	exists: bool,
	highest_index_yes_vote: u32,
}

// This pallet's events.
decl_event! {
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		Initialized(AccountId),
		CreateProposal(u32, AccountId),
		// SubmitVote(proposal_index, sender, unit_vote),
		CreateToken(u32, u32),
		Transfer(AccountId, AccountId, u64),
    }
}

// This pallet's errors.
decl_error! {
    pub enum Error for Module<T: Trait> {
		AlreadyInitialized,
		InsufficientFunds,
		Overflow,
		NotInit
    }
}
// This pallet's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as MolochTemplate {

		Members get(fn members): Vec<Member>;

		Proposals get(fn proposals): map hasher(blake2_128_concat) ProposalIndex => ProposalInfo<T::AccountId, T::BlockNumber>;

		ProposalCount get(fn proposal_count): ProposalIndex;

		Tokens get(fn tokens): map hasher(blake2_128_concat) TokenIndex => TokenCreation;

		TokenCount get(fn token_count): TokenIndex;
		
	 	Balances get(fn get_balance): map hasher(blake2_128_concat) T::AccountId => u64;

        TotalSupply get(fn total_supply): u64 = 21000000;

		DidInit get(fn is_init): bool;
    }
}
// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing errors
        type Error = Error<T>;

        // A default function for depositing events
        fn deposit_event() = default;

		// const StartingPeriod: T::BlockNumber = T::StartingPeriod::get();

		// const VotingPeriod: T::BlockNumber = T::VotingPeriod::get();

		#[weight = 10_000]
		fn init(origin) {
			let sender = ensure_signed(origin)?;
			ensure!(!DidInit::exists(), <Error<T>>::AlreadyInitialized);

			// TODO. Rewrite this part
			let mut current_members = Self::members();

			let new_member = Member {
				exists: true,
				highest_index_yes_vote: 0,
			};

			current_members.insert(0, new_member);
			Members::put(current_members);
			<Balances<T>>::insert(sender, Self::total_supply());
			DidInit::put(true);
		}

		#[weight = 10_000]
		fn create_proposal(origin) {
			let sender = ensure_signed(origin)?;
			ensure!(DidInit::exists() && Self::is_init(), <Error<T>>::NotInit);

			let starting_period;
			starting_period = <system::Module<T>>::block_number() + T::StartingPeriod::get();

			let index = ProposalCount::get();
			let next_index = index.checked_add(1).ok_or(Error::<T>::Overflow)?;
			ProposalCount::put(next_index);

			let index_token = TokenCount::get();
			let following_index = index_token.checked_add(1).ok_or(Error::<T>::Overflow)?;
			TokenCount::put(following_index);

			let new_proposal = ProposalInfo {
				proposer: sender.clone(),
				starting_period: starting_period,
				// TODO. Write another module for the token creation
				yes_votes: 0,
				no_votes: 0,
				processed: false,
				did_pass: false,
				aborted: false,
			};

			let new_tokens = TokenCreation {
				tribute_token: 0,
				payment_token: 0,
			};

			<Tokens>::insert(index_token, new_tokens);
			<Proposals<T>>::insert(index, new_proposal);
			Self::deposit_event(RawEvent::CreateProposal(index, sender));
			//Self::deposit_event(RawEvent::CreateToken(index, sender));
		}

		#[weight = 10_000]
		fn voting_period(origin) {
			let sender = ensure_signed(origin)?;

			let starting_period = <system::Module<T>>::block_number() + T::StartingPeriod::get();

			let can_vote;

			// TODO. Have the starting period available for only 7 days - BUG HERE
			if starting_period >= 7 {
				can_vote = false;
			}
			else {
				can_vote = true;
			}
		}

		/*
			Members can submit votes on proposals that were submitted, 
			proposals can pass if there is a majority vote, even if only 
			one person voted. 
		*/
		#[weight = 10_000]
		fn submit_vote(origin, unit_vote: u8){
			let sender = ensure_signed(origin)?;

			let starting_period = <system::Module<T>>::block_number() + T::StartingPeriod::get();

			let voting_expired_period = starting_period + T::VotingPeriod::get();

			let proposal_index = ProposalCount::get();

			let member = <Members>::get();

			let proposal = <ProposalInfo>::get();

			// TODO - grab the value of the votes from the struct properly
			let mut yesVotes = Self::proposal.yes_votes;
			let mut noVotes = Self::proposal.no_votes;

			// ensure!(unit_vote == 0 || unit_vote == 1, "Vote must be either 0(Yes) or 1(No)");

			// Vote is counted as yes (0)
			if unit_vote == 0 {
				// Increase value of total yes votes for a specific proposal
				// (ProposalInfo.yes_votes : ProposalInfo).yes_votes + 1;
				yesVotes += 1;
				// if proposal_index >= member.highest_index_yes_vote {
				// 	member.highest_index_yes_vote = proposal_index;
				// }
			// Vote is counted as no (0)
			} else {
				noVotes += 1;
			};

			//Self::deposit_event(RawEvent::SubmitVote(proposal_index, sender, unit_vote));
		}

		// if no vote, account take back shares, loss in voting power
		// if account take back all shares, no longer a member
		#[weight = 10_000]
		fn rage_quit(origin) {
			// let sender = ensure_signed(origin)?;
			// let voting_power = bool;

			// // heavy refactor
			// if member && yes_votes && no_votes == 0 {
			// 	voting_power = false;
			// } 
			// else {
			// 	voting_power = false;
			// 	let member = Member {
			// 		exists: false,
			// 		highest_index_yes_vote: 0,
			// 	};
			// }
		}

		/*
			Members can also propose trades to swap tokens OTC with the guild bank, 
			which could be used for making investments, active portfolio management, 
			selloffs, or just to top off a stablecoin reserve used to pay for planned expenses
		*/
		#[weight = 10_000]
		fn Transfer(_origin, to: T::AccountId, value: u64) {
			let sender = ensure_signed(_origin)?;
			let sender_balance = Self::get_balance(&sender);

			//Receiver balance reffering to 'guild bank'
			let receiver_balance = Self::get_balance(&to);

			let updated_from_balance = sender_balance.checked_sub(value).ok_or(<Error<T>>::InsufficientFunds)?;
			let updated_to_balance = receiver_balance.checked_add(value).expect("Entire supply fits in u64; qed");

			<Balances<T>>::insert(&sender, updated_from_balance);
			<Balances<T>>::insert(&to, updated_to_balance);

			Self::deposit_event(RawEvent::Transfer(sender, to, value));
		}
    }
}