//! # Roles Pallet
//!
//! The Roles Pallet is used For User's Account creation in the FairSquares framework
//!
//! ## Overview
//!
//! The Roles Pallet provides account management capabilities through the following actions:
//! - Account creation
//! - Roles selection
//! - Account creation approval or rejection
//! During Account creation, the user selects a role (or account type) from the Accounts enum. each
//! role has access to specific set of actions used in Fairsquares. there are currently 5 kinds of
//! roles available for selection:
//! - INVESTOR
//! - TENANT
//! - SERVICER
//! - SELLER
//! The 5th role which is the accounts administrator role is not available during account creation.
//! Sellers and Servicers accounts must be verified/approved by an administrator in order to become
//! active
//!
//! ### Dispatchable Functions
//! #### Account creation
//! * `create_account` - Create one of the 4 selectable type of account/role.
//! In the case of Sellers and Servicers, requests are transfered to a Role approval list
//!
//! #### Account management by Administrator
//! * `account_approval` - This function allows the administrator to verify/approve Seller and
//!   Servicer accounts creation requests
//! that are in the approval list.
//! Verified accounts are activated, i.e., tranfered to the corresponding role storage
//!
//! * `account_rejection` - This function allows the administrator to reject Seller and Servicer
//!   accounts creation requests
//! that are in the approval list, but do not fullfill the FaiSquares guideline.
//!
//! * `set_manager` - This function allows the current manager to tranfer his Administrative
//!   authority to a different user/account.
//! Only the current manager can use this function, and he will lose all administrative power by
//! using this function.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod helpers;
mod structs;

pub use crate::structs::*;
pub use pallet_sudo as SUDO;
#[frame_support::pallet]
pub mod pallet {
	pub use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + SUDO::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: ReservableCurrency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn investors)]
	///Registry of Investors organized by AccountId
	pub(super) type InvestorLog<T: Config> =
		StorageMap<_, Twox64Concat, AccountIdOf<T>, Investor<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn sellers)]
	///Registry of Sellers organized by AccountId
	pub(super) type HouseSellerLog<T: Config> =
		StorageMap<_, Twox64Concat, AccountIdOf<T>, HouseSeller<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn tenants)]
	///Registry of Tenants organized by AccountId
	pub(super) type TenantLog<T: Config> =
		StorageMap<_, Twox64Concat, AccountIdOf<T>, Tenant<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn servicers)]
	///Registry of Servicers organized by AccountId
	pub(super) type ServicerLog<T: Config> =
		StorageMap<_, Twox64Concat, AccountIdOf<T>, Servicer<T>, OptionQuery>;

	#[pallet::type_value]
	///Initializing function for the approval waiting list
	pub(super) fn MyDefault<T: Config>() -> Idle<T> {
		(Vec::new(), Vec::new())
	}
	#[pallet::storage]
	#[pallet::getter(fn get_pending_approvals)]
	///Approval waiting list for Sellers and Servicers
	pub(super) type RoleApprovalList<T: Config> =
		StorageValue<_, Idle<T>, ValueQuery, MyDefault<T>>;

	#[pallet::storage]
	#[pallet::getter(fn get_roles)]
	///Registry of Roles by AccountId
	pub(super) type AccountsRolesLog<T: Config> =
		StorageMap<_, Twox64Concat, AccountIdOf<T>, Accounts, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		InvestorCreated(T::BlockNumber, T::AccountId),
		TenantCreated(T::BlockNumber, T::AccountId),
		SellerCreated(T::BlockNumber, T::AccountId),
		ServicerCreated(T::BlockNumber, T::AccountId),
		AccountCreationApproved(T::BlockNumber, T::AccountId),
		AccountCreationRejected(T::BlockNumber, T::AccountId),
		SellerAccountCreationRejected(T::BlockNumber, T::AccountId),
		ServicerAccountCreationRejected(T::BlockNumber, T::AccountId),
		CreationRequestCreated(T::BlockNumber, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		/// Error on initialization.
		InitializationError,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		///One role is allowed
		OneRoleAllowed,
		///Invalid Operation
		InvalidOperation,
		///Require Sudo
		RequireSudo,
		/// Account already in the waiting list
		AlreadyWaiting,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		///Account creation function. Only one role per account is permitted.
		pub fn create_account(origin: OriginFor<T>, account_type: Accounts) -> DispatchResult {
			let caller = ensure_signed(origin.clone())?;
			match account_type {
				Accounts::INVESTOR => {
					Self::check_storage(caller.clone())?;
					let _acc =
						Investor::<T>::new(origin).map_err(|_| <Error<T>>::InitializationError)?;
					let now = <frame_system::Pallet<T>>::block_number();
					AccountsRolesLog::<T>::insert(&caller, Accounts::INVESTOR);
					Self::deposit_event(Event::InvestorCreated(now, caller));
					Ok(().into())
				},
				Accounts::SELLER => {
					Self::check_storage(caller.clone())?;
					Self::check_role_approval_list(caller.clone())?;
					let _acc = HouseSeller::<T>::new(origin)
						.map_err(|_| <Error<T>>::InitializationError)?;
					let now = <frame_system::Pallet<T>>::block_number();
					Self::deposit_event(Event::CreationRequestCreated(now, caller));
					Ok(().into())
				},
				Accounts::TENANT => {
					Self::check_storage(caller.clone())?;
					let _acc =
						Tenant::<T>::new(origin).map_err(|_| <Error<T>>::InitializationError)?;
					let now = <frame_system::Pallet<T>>::block_number();
					AccountsRolesLog::<T>::insert(&caller, Accounts::TENANT);
					Self::deposit_event(Event::TenantCreated(now, caller));
					Ok(().into())
				},
				Accounts::SERVICER => {
					Self::check_storage(caller.clone())?;
					Self::check_role_approval_list(caller.clone())?;
					let _acc =
						Servicer::<T>::new(origin).map_err(|_| <Error<T>>::InitializationError)?;
					let now = <frame_system::Pallet<T>>::block_number();
					Self::deposit_event(Event::CreationRequestCreated(now, caller));
					Ok(().into())
				},
			}
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		///Approval function for Sellers and Servicers. Only for admin level.
		pub fn account_approval(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;
			ensure!(
				sender == SUDO::Pallet::<T>::key().unwrap(),
				"only the current sudo key can sudo"
			);
			Self::approve_account(account.clone())?;
			let now = <frame_system::Pallet<T>>::block_number();
			Self::deposit_event(Event::AccountCreationApproved(now, account));
			Ok(().into())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		///Creation Refusal function for Sellers and Servicers. Only for admin level.
		pub fn account_rejection(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;
			ensure!(
				sender == SUDO::Pallet::<T>::key().unwrap(),
				"only the current sudo key can sudo"
			);
			Self::reject_account(account.clone())?;
			let now = <frame_system::Pallet<T>>::block_number();
			Self::deposit_event(Event::AccountCreationRejected(now, account));
			Ok(().into())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		///The caller will transfer his admin authority to a different account
		pub fn set_manager(
			origin: OriginFor<T>,
			new: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;
			ensure!(
				sender == SUDO::Pallet::<T>::key().unwrap(),
				"only the current sudo key can sudo"
			);
			SUDO::Pallet::<T>::set_key(origin, new).ok();
			Ok(().into())
		}
	}
}
