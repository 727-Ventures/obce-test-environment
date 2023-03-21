use obce::substrate::{
	frame_system::{Config as SysConfig, RawOrigin},
	pallet_contracts::{chain_extension::Ext, Config as ContractConfig},
	sp_runtime::traits::StaticLookup,
	ChainExtensionEnvironment,
	ExtensionContext,
};
use pallet_example::{self, Pallet};

use crate::{ChainExtension, Error};

#[derive(Default)]
pub struct Extension {}

#[obce::implementation]
impl<'a, E, T, Env> ChainExtension<T> for ExtensionContext<'a, E, T, Env, Extension>
where
	T: SysConfig + ContractConfig + pallet_example::Config,
	<<T as SysConfig>::Lookup as StaticLookup>::Source: From<<T as SysConfig>::AccountId>,
	E: Ext<T = T>,
	Env: ChainExtensionEnvironment<E, T>,
{
	#[obce(weight(dispatch = "pallet_example::Pallet::<T>::successful_method"))]
	fn successful_method(&mut self) -> Result<(), Error<T>> {
		Ok(Pallet::<T>::successful_method(
			RawOrigin::Signed(self.env.ext().address().clone()).into(),
		)?)
	}

	#[obce(weight(dispatch = "pallet_example::Pallet::<T>::erroneous_method"))]
	fn erroneous_method(&mut self) -> Result<(), Error<T>> {
		Ok(Pallet::<T>::erroneous_method(
			RawOrigin::Signed(self.env.ext().address().clone()).into(),
		)?)
	}

	#[obce(weight(dispatch = "pallet_example::Pallet::<T>::critically_erroneous_method"))]
	fn critically_erroneous_method(&mut self) -> Result<(), Error<T>> {
		Ok(Pallet::<T>::critically_erroneous_method(
			RawOrigin::Signed(self.env.ext().address().clone()).into(),
		)?)
	}

	fn multi_arg_method(&mut self, one: u64, two: u64) -> Result<u64, Error<T>> {
		Ok(one + two)
	}

	#[obce(weight(dispatch = "pallet_example::Pallet::<T>::weight_linear_method"))]
	fn weight_linear_method(&mut self, complexity: u64) -> Result<(), Error<T>> {
		Ok(Pallet::<T>::weight_linear_method(
			RawOrigin::Signed(self.env.ext().address().clone()).into(),
			complexity,
		)?)
	}
}
