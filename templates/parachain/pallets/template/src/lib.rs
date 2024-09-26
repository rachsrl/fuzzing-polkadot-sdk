//! # Template Pallet
//!
//! A pallet with minimal functionality to help developers understand the essential components of
//! writing a FRAME pallet. It is typically used in beginner tutorials or in Polkadot SDK template
//! as a starting point for creating a new pallet and **not meant to be used in production**.
//!
//! ## Overview
//!
//! This template pallet contains basic examples of:
//! - declaring a storage item that stores a single block-number
//! - declaring and using events
//! - declaring and using errors
//! - a dispatchable function that allows a user to set a new value to storage and emits an event
//!   upon success
//! - another dispatchable function that causes a custom error to be thrown
//!
//! Each pallet section is annotated with an attribute using the `#[pallet::...]` procedural macro.
//! This macro generates the necessary code for a pallet to be aggregated into a FRAME runtime.
//!
//! To get started with pallet development, consider using this tutorial:
//!
//! <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html>
//!
//! And reading the main documentation of the `frame` crate:
//!
//! <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/polkadot_sdk/frame_runtime/index.html>
//!
//! And looking at the frame [`kitchen-sink`](https://paritytech.github.io/polkadot-sdk/master/pallet_example_kitchensink/index.html)
//! pallet, a showcase of all pallet macros.
//!
//! ### Pallet Sections
//!
//! The pallet sections in this template are:
//!
//! - A **configuration trait** that defines the types and parameters which the pallet depends on
//!   (denoted by the `#[pallet::config]` attribute). See: [`Config`].
//! - A **means to store pallet-specific data** (denoted by the `#[pallet::storage]` attribute).
//!   See: [`storage_types`].
//! - A **declaration of the events** this pallet emits (denoted by the `#[pallet::event]`
//!   attribute). See: [`Event`].
//! - A **declaration of the errors** that this pallet can throw (denoted by the `#[pallet::error]`
//!   attribute). See: [`Error`].
//! - A **set of dispatchable functions** that define the pallet's functionality (denoted by the
//!   `#[pallet::call]` attribute). See: [`dispatchables`].
//!
//! Run `cargo doc --package pallet-template --open` to view this pallet's documentation.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use codec::{Encode, Decode};
use sp_runtime::RuntimeDebug;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html>
//
// To see a full list of `pallet` macros and their use cases, see:
// <https://paritytech.github.io/polkadot-sdk/master/pallet_example_kitchensink/index.html>
// <https://paritytech.github.io/polkadot-sdk/master/frame_support/pallet_macros/index.html>
#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*, DefaultNoBound};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::{CheckedAdd, One};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		/// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_runtime_types/index.html>
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: crate::weights::WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// A struct to store a single block-number. Has all the right derives to store it in storage.
	/// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_storage_derives/index.html>
	#[derive(
		Encode, Decode, MaxEncodedLen, TypeInfo, CloneNoBound, PartialEqNoBound, DefaultNoBound,
	)]
	#[scale_info(skip_type_params(T))]
	pub struct CompositeStruct<T: Config> {
		/// A block number.
		pub(crate) block_number: BlockNumberFor<T>,
	}

	// Define the Proposal struct
	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, RuntimeDebug, PartialEq, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Proposal {
    		pub votes_for: u32,
    		pub votes_against: u32,
	}

	/// The pallet's storage items.
	/// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#storage>
	/// <https://paritytech.github.io/polkadot-sdk/master/frame_support/pallet_macros/attr.storage.html>
	#[pallet::storage]
	pub type Something<T: Config> = StorageValue<_, CompositeStruct<T>>;
	/// Storage item for voting on proposals functionality
	#[pallet::storage]
	#[pallet::getter(fn proposals)]
	pub type Proposals<T> = StorageMap<_, Blake2_128Concat, u32, Proposal>;

	/// Pallets use events to inform users when important changes are made.
	/// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error>
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// We usually use passive tense for events.
		SomethingStored { block_number: BlockNumberFor<T>, who: T::AccountId },
		/// Event for vote casted
		// VoteCast { who: T::AccountId, proposal_id: Proposals<T>, vote_value: u32}, 
		VoteCast(T::AccountId, u32, bool),
	}


	/// Errors inform users that something went wrong.
	/// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error>
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Error to throw in case of proposal not found
		ProposalNotFound,
		/// Error to throw when Vote overflows
		VoteOverflow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	/// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	/// These functions materialize as "extrinsics", which are often compared to transactions.
	/// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	/// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#dispatchables>
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, bn: u32) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_origin/index.html>
			let who = ensure_signed(origin)?;

			// Convert the u32 into a block number. This is possible because the set of trait bounds
			// defined in [`frame_system::Config::BlockNumber`].
			let block_number: BlockNumberFor<T> = bn.into();

			// Update storage.
			<Something<T>>::put(CompositeStruct { block_number });

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { block_number, who });

			// Return a successful [`DispatchResultWithPostInfo`] or [`DispatchResult`].
			Ok(().into())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(mut old) => {
					// Increment the value read from storage; will error in the event of overflow.
					old.block_number = old
						.block_number
						.checked_add(&One::one())
						// ^^ equivalent is to:
						// .checked_add(&1u32.into())
						// both of which build a `One` instance for the type `BlockNumber`.
						.ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(old);
					// Explore how you can rewrite this using
					// [`frame_support::storage::StorageValue::mutate`].
					Ok(().into())
				},
			}
		}

		/// Allows a user to cast a vote on a specified proposal.
		/// Increments the vote count for the proposal based on the user's vote value,
		/// where `votes_for` indicates support for the proposal and `votes_against` indicates opposition.
                #[pallet::call_index(2)]
    		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn vote(origin: OriginFor<T>, proposal_id: u32, vote_value: bool) -> DispatchResult {
			let voter = ensure_signed(origin)?;
            		let mut proposal = Proposals::<T>::get(proposal_id).unwrap();

            		if vote_value {
            			proposal.votes_for = proposal.votes_for.checked_add(1).ok_or(Error::<T>::VoteOverflow)?;
			} else {
            			proposal.votes_against = proposal.votes_against.checked_add(1).ok_or(Error::<T>::VoteOverflow)?;
			}

            		// Save the updated proposal back to storage
           	 	Proposals::<T>::insert(proposal_id, proposal);

            		// Emit the event
	    		Self::deposit_event(Event::VoteCast(voter, proposal_id, vote_value));
	    		Ok(())
		}	
	}
}
