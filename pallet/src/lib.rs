#![cfg_attr(not(feature = "std"), no_std)]

mod weights;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::OriginFor;

	use crate::weights::WeightInfo;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::test_method(*_value))]
		pub fn test_method(_origin: OriginFor<T>, _value: u64) -> DispatchResult {
			Ok(())
		}
	}
}
