//! # Roles Pallet
//!
//! The Roles Pallet is used to set a role for a given AccountId in the FairSquares framework
//!
//! ## Overview
//!
//! The Roles Pallet provides roles management capabilities through the following actions:
//! - Role setting
//! - Role attribution to an AccountId
//! - Role attribution approval or rejection
//! During role setting, the user selects a role from the Accounts enum. Each
//! role has access to specific set of actions used in Fairsquares. there are currently 5 kinds of
//! roles available for selection:
//! - INVESTOR
//! - TENANT
//! - SERVICER
//! - SELLER
//! The 5th role which is the accounts administrator role is not available during role setting.
//! Sellers and Servicers roles, must be verified/approved by an administrator in order to become
//! active
//!
//! ### Dispatchable Functions
//! #### Role setting
//! * `set_role` - Create one of the 4 selectable type of role.
//! In the case of Sellers and Servicers, requests are transfered to a Role approval list.
//! Servicer role (and only Servicer role) can also assign roles to a different user account.
//!
//! #### Roles management by Administrator
//! * `account_approval` - This function allows the administrator to verify/approve Seller and
//! Servicer role connection to the requesting AccountId.
//! Verified AccountId are activated, i.e., the requesting AccountId is stored into the
//! corresponding role storage.
//!
//! * `account_rejection` - This function allows the administrator to reject Seller and Servicer
//! role connection to the requesting AccountId
//! that are in the approval list, but do not fullfill the FaiSquares guideline.
//!
//! * `set_manager` - This function allows the current manager/Sudo_Account to transfer his
//!   Administrative
//! authority to a different user/account.
//! Only the current manager can use this function, and he will lose all administrative power by
//! using this function. The Servicer Role is affected to new manager account during this transfer.
//! Previous manager account Servicer Role is revoked.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod functions;
mod types;
pub mod weights;
pub use crate::types::*;
pub use pallet_sudo as SUDO;
use sp_std::prelude::*;
pub use weights::WeightInfo;
#[frame_support::pallet]
pub mod pallet {
	pub use super::*;
	//use frame_system::WeightInfo;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + SUDO::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: ReservableCurrency<Self::AccountId>;
		type WeightInfo: WeightInfo;

		#[pallet::constant]
		type MaxMembers: Get<u32>;
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
	#[pallet::getter(fn notaries)]
	pub(super) type NotaryLog<T: Config> =
		StorageMap<_, Twox64Concat, AccountIdOf<T>, Notary<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn reps)]
	///Registry of Sellers organized by AccountId
	pub type RepresentativeLog<T: Config> =
		StorageMap<_, Twox64Concat, AccountIdOf<T>, Representative<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn tenants)]
	///Registry of Tenants organized by AccountId
	pub type TenantLog<T: Config> =
		StorageMap<_, Twox64Concat, AccountIdOf<T>, Tenant<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn servicers)]
	///Registry of Servicers organized by AccountId
	pub(super) type ServicerLog<T: Config> =
		StorageMap<_, Twox64Concat, AccountIdOf<T>, Servicer<T>, OptionQuery>;

	#[pallet::type_value]
	/// Initializer for the approval list of house sellers
	pub(super) fn InitPendingSellerList<T: Config>() -> Vec<HouseSeller<T>> {
		Vec::new()
	}

	#[pallet::type_value]
	/// Initializer for the approval list of servicers
	pub(super) fn InitPendingServicerList<T: Config>() -> Vec<Servicer<T>> {
		Vec::new()
	}

	#[pallet::type_value]
	/// Initializer for the approval list of notaries
	pub(super) fn InitPendingNotaryList<T: Config>() -> Vec<Notary<T>> {
		Vec::new()
	}

	#[pallet::type_value]
	/// Initializer for the approval list of representatives
	pub(super) fn InitRepApprovalList<T: Config>() -> Vec<Representative<T>> {
		Vec::new()
	}

	#[pallet::storage]
	#[pallet::getter(fn get_pending_house_sellers)]
	pub(super) type SellerApprovalList<T: Config> =
		StorageValue<_, Vec<HouseSeller<T>>, ValueQuery, InitPendingSellerList<T>>;

	#[pallet::storage]
	#[pallet::getter(fn get_pending_servicers)]
	pub(super) type ServicerApprovalList<T: Config> =
		StorageValue<_, Vec<Servicer<T>>, ValueQuery, InitPendingServicerList<T>>;

	#[pallet::storage]
	#[pallet::getter(fn get_pending_notaries)]
	pub(super) type NotaryApprovalList<T: Config> =
		StorageValue<_, Vec<Notary<T>>, ValueQuery, InitPendingNotaryList<T>>;

	#[pallet::storage]
	#[pallet::getter(fn get_pending_representatives)]
	///Approval waiting list for Representatives
	pub type RepApprovalList<T: Config> =
		StorageMap<_, Twox64Concat, AccountIdOf<T>, Representative<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_roles)]
	///Registry of Roles by AccountId
	pub type AccountsRolesLog<T: Config> =
		StorageMap<_, Twox64Concat, AccountIdOf<T>, Accounts, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_requested_role)]
	pub type RequestedRoles<T: Config> =
		StorageMap<_, Twox64Concat, AccountIdOf<T>, Accounts, OptionQuery>;

	#[pallet::type_value]
	///Initializing function for the total number of members
	pub(super) fn InitTotalMembers() -> u32 {
		0
	}

	#[pallet::storage]
	#[pallet::getter(fn total_members)]
	pub(super) type TotalMembers<T: Config> = StorageValue<_, u32, ValueQuery, InitTotalMembers>;

	#[pallet::type_value]
	///Initializing function for the total number of Rep members
	pub fn InitRepMembers() -> u32 {
		0
	}

	#[pallet::storage]
	#[pallet::getter(fn rep_num)]
	///Number of active Representative
	pub type RepNumber<T: Config> = StorageValue<_, u32, ValueQuery, InitRepMembers>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub new_admin: Option<T::AccountId>,
		pub representatives: Vec<T::AccountId>,
	}
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { new_admin: Default::default(), representatives: vec![] }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			if self.new_admin.is_some() {
				let servicer0 = self.new_admin.clone().unwrap(); // AccountId
				let origin = T::Origin::from(RawOrigin::Signed(servicer0.clone())); //Origin
				let source = T::Lookup::unlookup(servicer0); //Source
				crate::Pallet::<T>::set_manager(origin, source).ok();
			}

			if !self.representatives.is_empty() {
				crate::Pallet::<T>::init_representatives(self.representatives.clone());
			}
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Investor role successfully attributed
		InvestorCreated(T::BlockNumber, T::AccountId),
		/// Tenant role successfully attributed
		TenantCreated(T::BlockNumber, T::AccountId),
		/// Seller role successfully attributed
		SellerCreated(T::BlockNumber, T::AccountId),
		/// Servicer role successfully attributed
		ServicerCreated(T::BlockNumber, T::AccountId),
		/// Notary role successfully attributed
		NotaryCreated(T::BlockNumber, T::AccountId),
		/// Request for new role accepted
		AccountCreationApproved(T::BlockNumber, T::AccountId),
		/// Request for new role Rejected
		AccountCreationRejected(T::BlockNumber, T::AccountId),
		/// Seller role request rejected
		SellerAccountCreationRejected(T::BlockNumber, T::AccountId),
		/// Servicer role request rejected
		ServicerAccountCreationRejected(T::BlockNumber, T::AccountId),
		/// Notary role request rejected
		NotaryAccountCreationRejected(T::BlockNumber, T::AccountId),
		/// Role request added to the role approval waiting list
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
		///Account is not in waiting list
		NotInWaitingList,
		/// Account already in the waiting list
		AlreadyWaiting,
		///Maximum limit for number of members exceeded
		TotalMembersExceeded,
		/// Action reserved to servicers
		OnlyForServicers,
		/// Cannot do the approval or rejection
		UnAuthorized,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(<T as pallet::Config>::WeightInfo::investor(4))]
		///Account creation function. Only one role per account is permitted.
		pub fn set_role(
			origin: OriginFor<T>,
			account: AccountIdOf<T>,
			account_type: Accounts,
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			if caller != account {
				ensure!(ServicerLog::<T>::contains_key(&caller), Error::<T>::OnlyForServicers);
			}
			Self::check_account_role(account.clone())?;
			let now = <frame_system::Pallet<T>>::block_number();
			let members = Self::total_members();
			let requested = Self::get_requested_role(&account).is_some();
			match account_type {
				Accounts::INVESTOR => {
					let investor = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(
						account.clone(),
					));
					Investor::<T>::new(investor).map_err(|_| <Error<T>>::InitializationError)?;
					AccountsRolesLog::<T>::insert(&account, Accounts::INVESTOR);
					Self::increase_total_members().ok();
					Self::deposit_event(Event::InvestorCreated(now, account.clone()));
				},
				Accounts::SELLER => {
					ensure!(!requested, <Error<T>>::AlreadyWaiting);
					let seller = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(
						account.clone(),
					));
					HouseSeller::<T>::new(seller).map_err(|_| <Error<T>>::InitializationError)?;
					Self::deposit_event(Event::CreationRequestCreated(now, account.clone()));
				},
				Accounts::TENANT => {
					let tenant = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(
						account.clone(),
					));
					Tenant::<T>::new(tenant).map_err(|_| <Error<T>>::InitializationError)?;
					AccountsRolesLog::<T>::insert(&account, Accounts::TENANT);
					Self::increase_total_members().ok();
					Self::deposit_event(Event::TenantCreated(now, account.clone()));
				},
				Accounts::SERVICER => {
					ensure!(!requested, <Error<T>>::AlreadyWaiting);
					let servicer = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(
						account.clone(),
					));
					Servicer::<T>::new(servicer).map_err(|_| <Error<T>>::InitializationError)?;
					Self::deposit_event(Event::CreationRequestCreated(now, account.clone()));
				},
				Accounts::NOTARY => {
					ensure!(!requested, <Error<T>>::AlreadyWaiting);
					let notary = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(
						account.clone(),
					));
					Notary::<T>::new(notary).map_err(|_| <Error<T>>::InitializationError)?;
					Self::deposit_event(Event::CreationRequestCreated(now, account.clone()));
				},
				Accounts::REPRESENTATIVE => {
					//ensure!(!requested, <Error<T>>::AlreadyWaiting);
					let representative = <T as frame_system::Config>::Origin::from(
						RawOrigin::Signed(account.clone()),
					);
					Representative::<T>::new(representative)
						.map_err(|_| <Error<T>>::InitializationError)?;
					Self::deposit_event(Event::CreationRequestCreated(now, account.clone()));
				},
			}

			let need_approval = !matches!(
				account_type,
				Accounts::INVESTOR | Accounts::TENANT | Accounts::REPRESENTATIVE
			);
			if need_approval {
				RequestedRoles::<T>::insert(&account, account_type);
			} else {
				TotalMembers::<T>::put(members + 1);
			}

			Ok(())
		}

		#[pallet::weight(<T as pallet::Config>::WeightInfo::approval(5))]
		///Approval function for Sellers, Servicers, and Notary. Only for admin level.
		pub fn account_approval(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(
				sender == SUDO::Pallet::<T>::key().unwrap(),
				"only the current sudo key can sudo"
			);

			let role = Self::get_requested_role(&account);
			ensure!(role.is_some(), Error::<T>::NotInWaitingList);

			ensure!(role != Some(Accounts::REPRESENTATIVE), Error::<T>::UnAuthorized);

			Self::approve_account(sender, account.clone())?;
			let now = <frame_system::Pallet<T>>::block_number();
			Self::deposit_event(Event::AccountCreationApproved(now, account));
			Ok(())
		}

		#[pallet::weight(<T as pallet::Config>::WeightInfo::rejection(6))]
		///Creation Refusal function for Sellers and Servicers. Only for admin level.
		pub fn account_rejection(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(
				sender == SUDO::Pallet::<T>::key().unwrap(),
				"only the current sudo key can sudo"
			);

			let role = Self::get_requested_role(&account);
			ensure!(role.is_some(), Error::<T>::NotInWaitingList);

			// We can't reject a representive role request
			ensure!(role != Some(Accounts::REPRESENTATIVE), Error::<T>::UnAuthorized);
			Self::reject_account(account.clone())?;

			RequestedRoles::<T>::remove(&account);

			let now = <frame_system::Pallet<T>>::block_number();
			Self::deposit_event(Event::AccountCreationRejected(now, account));
			Ok(())
		}

		#[pallet::weight(<T as pallet::Config>::WeightInfo::set_admin(7))]
		///The caller will transfer his admin authority to a different account
		pub fn set_manager(
			origin: OriginFor<T>,
			new: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;
			let new0 = T::Lookup::lookup(new.clone())?;
			let new_origin = T::Origin::from(RawOrigin::Signed(new0.clone()));
			ensure!(
				sender == SUDO::Pallet::<T>::key().unwrap(),
				"only the current sudo key can sudo"
			);
			//ensure!(sender != new0, "The same manager is given");
			//Remove current Sudo from Servicers list
			if ServicerLog::<T>::contains_key(sender.clone()) {
				ServicerLog::<T>::remove(sender.clone());
			}

			//create Servicer & approve a servicer account for new Sudo
			//if the new Sudo has no role yet
			if !AccountsRolesLog::<T>::contains_key(&new0) {
				Servicer::<T>::new(new_origin).ok();
				Self::approve_account(sender, new0).ok();
			}
			SUDO::Pallet::<T>::set_key(origin, new).ok();
			Ok(())
		}
	}
}
