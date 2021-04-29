//! Minimal Pallet for parachain staking

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet;
pub use pallet::*;

#[pallet]
pub mod pallet {

    /// Pallet for parachain staking
    #[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);

}