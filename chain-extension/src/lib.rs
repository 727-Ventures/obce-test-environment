#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "substrate")]
use obce::substrate::{frame_support::traits::PalletInfoAccess, CriticalError};
#[cfg(feature = "substrate")]
use pallet_example::Error as PalletError;

/// Implementation of the `PalletAssets` trait for the chain extension on `substrate` level.
#[cfg(feature = "substrate")]
pub mod substrate;

#[cfg(feature = "ink")]
pub mod ink;

#[obce::error]
pub enum Error<T> {
	NonCriticalError,

	#[obce(critical)]
	Critical(CriticalError),

	#[doc(hidden)]
	#[codec(skip)]
	__Ignore(core::marker::PhantomData<T>),
}

#[cfg(feature = "substrate")]
impl<T: pallet_example::Config> From<CriticalError> for Error<T> {
	fn from(dispatch: CriticalError) -> Self {
		let asset_module = <pallet_example::Pallet<T> as PalletInfoAccess>::index() as u8;

		if let CriticalError::Module(module) = dispatch {
			if module.index == asset_module {
				let mut input = module.error.as_slice();
				if let Ok(_) = <PalletError<T> as scale::Decode>::decode(&mut input) {
					return Error::NonCriticalError
				}
			}
		}

		Error::Critical(dispatch)
	}
}

#[obce::definition(id = "obce-chain-extension-example@v0.1")]
pub trait ChainExtension<T> {
	#[obce(id = 0x123)]
	fn successful_method(&mut self) -> Result<(), Error<T>>;

	#[obce(id = "erroneous_method_identifier")]
	fn erroneous_method(&mut self) -> Result<(), Error<T>>;

	fn critically_erroneous_method(&mut self) -> Result<(), Error<T>>;

	fn multi_arg_method(&mut self, one: u64, two: u64) -> Result<u64, Error<T>>;

	fn weight_linear_method(&mut self, complexity: u64) -> Result<(), Error<T>>;
}
