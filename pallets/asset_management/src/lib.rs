#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub use pallet_roles as Roles;
pub use pallet_democracy as Dem;
pub use pallet_share_distributor as Share;
pub use pallet_nft as Nft;
pub use pallet_onboarding as Onboarding;
pub use pallet_housing_fund as HFund;

mod functions;
mod types;
pub use crate::types::*;
pub use functions::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

//#[cfg(feature = "runtime-benchmarks")]
//mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_system::WeightInfo;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + HFund::Config + Onboarding::Config +Roles::Config + Dem::Config + Share::Config + Nft::Config{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Call: Parameter
			+ UnfilteredDispatchable<Origin = <Self as frame_system::Config>::Origin>
			+ From<Call<Self>>
			+ GetDispatchInfo;
		type Delay: Get<Self::BlockNumber>;
		type CheckDelay: Get<Self::BlockNumber>;
		type InvestorVoteAmount: Get<u128>;
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
		type WeightInfo: WeightInfo;

		#[pallet::constant]
		type MinimumDepositVote: Get<BalanceOf<Self>>;

		#[pallet::constant]
		type CheckPeriod: Get<Self::BlockNumber>;
	}

	//Store the referendum_index and the struct containing the virtual_account/caller/potential_rep/vote_result
	#[pallet::storage]
	#[pallet::getter(fn proposals)]
	pub type ProposalsLog<T: Config> =
		StorageMap<_, Blake2_128Concat, Dem::ReferendumIndex, RepVote<T>, OptionQuery>;	

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		///A voting session to elect a representative has started
		RepresentativeVoteSessionStarted{
			caller: T::AccountId,
			candidate: T::AccountId,
			asset_account: T::AccountId,
		},


	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// The account is not an Asset account
		NotAnAssetAccount,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		///The proposal could not be created
		FailedToCreateProposal,
		///This Preimage already exists
		DuplicatePreimage,
		///Not an owner in the corresponding virtual account
		NotAnOwner,
		///The Asset Does not Exists
		NotAnAsset
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		///Owners Voting system
		///One owner trigger a vote session with a proposal
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn representative_session(origin:OriginFor<T>,asset_type: Nft::PossibleCollections, asset_id: T::NftItemId,representative: T::AccountId ) -> DispatchResult{
			let caller = ensure_signed(origin.clone())?;

			//Get the asset virtual account if it exists
			let collection_id: T::NftCollectionId = asset_type.value().into();
			let ownership = Share::Pallet::<T>::virtual_acc(collection_id,asset_id);
			ensure!(!ownership.clone().is_none(),Error::<T>::NotAnAsset);
			let virtual_account = ownership.clone().unwrap().virtual_account;

			//Ensure that the caller is an owner related to the virtual account
			let owners = ownership.unwrap().owners;
			ensure!(owners.contains(&caller),Error::<T>::NotAnOwner);

			//Make proposal
			let deposit = T::MinimumDeposit::get();

			//Create the call 
			let rep_call = Call::<T>::representative_approval {
				rep_account: representative.clone(),
				collection: collection_id,
				item: asset_id
			};
			
			//Create and add the proposal
			let prop_hash = Self::create_proposal_hash_and_note(caller.clone(),rep_call.into());
			Dem::Pallet::<T>::propose(origin,prop_hash,deposit).ok();

			let threshold = Dem::VoteThreshold::SimpleMajority;
			let delay = <T as Config>::Delay::get();
			let referendum_index =
			Dem::Pallet::<T>::internal_start_referendum(prop_hash, threshold, delay);

			//Create data for proposals Log
			RepVote::<T>::new(caller.clone(),virtual_account.clone(),representative.clone(),referendum_index).ok();
			
			Self::deposit_event(Event::RepresentativeVoteSessionStarted{
				caller: caller,
				candidate: representative,
				asset_account: virtual_account,
			});
			
			Ok(())
		}

		/// approve a Representative role request
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn representative_approval(origin: OriginFor<T>, rep_account: T::AccountId,collection: T::NftCollectionId,item: T::NftItemId) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			//Check that the caller is a stored virtual account
			ensure!(caller == Share::Pallet::<T>::virtual_acc(collection,item).unwrap().virtual_account, Error::<T>::NotAnAssetAccount);
			//Check that the account is in the representative waiting list
			ensure!(Roles::Pallet::<T>::get_pending_representatives(&rep_account).is_some(),"problem");
			//Approve role request
			Self::approve_representative(caller,rep_account).ok();

			Ok(())
		}


	}
}