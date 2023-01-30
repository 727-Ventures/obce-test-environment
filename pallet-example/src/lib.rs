#![cfg_attr(not(feature = "std"), no_std)]

mod weights;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::OriginFor;

	use crate::weights::WeightInfo;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type WeightInfo: WeightInfo;
	}

	#[pallet::error]
	pub enum Error<T> {
		SomeSpecificError,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::const_ref_time(0))]
		pub fn successful_method(_: OriginFor<T>) -> DispatchResult {
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::const_ref_time(0))]
		pub fn erroneous_method(_: OriginFor<T>) -> DispatchResult {
			Err(Error::<T>::SomeSpecificError.into())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::const_ref_time(0))]
		pub fn critically_erroneous_method(_: OriginFor<T>) -> DispatchResult {
			Err(DispatchError::Unavailable)
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::const_ref_time(*_complexity))]
		pub fn weight_linear_method(_: OriginFor<T>, _complexity: u64) -> DispatchResult {
			Ok(())
		}
	}
}
