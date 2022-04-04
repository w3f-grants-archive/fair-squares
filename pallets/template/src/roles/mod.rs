
mod items;
pub use super::*;
pub use crate::roles::items::*;
pub type BalanceOf<T> = <<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
pub type Contributors<T> = Vec<AccountIdOf<T>>;
pub type HouseIndex = u32;
pub type ContributionIndex = u32;
pub type Bool = bool;



#[derive(Debug, PartialEq, Encode, Decode)]
pub struct Investor<T:Config,U>{
    pub account_id:AccountIdOf<T>,
    pub nft:U,
    pub age:BlockNumberOf<T>,
}
impl<T:Config,U> Investor<T,U>{
    pub fn new(acc:AccountIdOf<T>,nft:U)-> Self{
	now = let now = <frame_system::Pallet<T>>::block_number();
        Investor{
            account_id: acc,
            nft: nft,
	    age: now,		
        }        
    }

}

impl<T:Config,U> Investor<T,U>{
    
    pub fn contribute(origin:OriginFor<T>,acc:AccountIdOf<T> ,value:BalanceOf<T>) -> DispatchResult{
        
        let who = ensure_signed(origin)?;
	ensure!(value >= T::MinContribution::get(), Error::<T>::ContributionTooSmall);
	
	let now = <frame_system::Pallet<T>>::block_number();
	let c1=Contribution::new(&acc,&value);
        if ContributionsLog::<T>::contains_key(c1.account){
            ContributionsLog::<T>::mutate(c1.account, |val|{
                *val += *c1.amount;
            })
        } else {
            ContributionsLog::<T>::insert(&acc,value);
        }
        

        <T as pallet::Config>::Currency::transfer(
            &who,
            &TREASURE_PALLET_ID.into_account(),
            value,
            ExistenceRequirement::AllowDeath,
        )?;

        Ok(().into())


    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct HouseOwner<T: Config,U>{
    pub account_id:T,
    pub nft:U,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct Tenant<T:Config,U>{
    pub account_id:AccountIdOf<T>,
    pub rent:U,
}
impl<T:Config,U> Tenant<T,U>{
    pub fn new(acc:AccountIdOf<T>,rent:U)-> Self{
        Tenant{
            account_id: acc,
            rent: rent,
        }
        
    }
}