use obce::substrate::{
	frame_system::{Config as SysConfig, RawOrigin},
	pallet_contracts::{chain_extension::Ext, Config as ContractConfig},
	sp_core::crypto::UncheckedFrom,
	sp_runtime::traits::StaticLookup,
	ExtensionContext,
};
use pallet::{self, Pallet};

use crate::ChainExtension;

#[derive(Default)]
pub struct Extension {}

#[obce::implementation]
impl<'a, 'b, E, T> ChainExtension for ExtensionContext<'a, 'b, E, T, Extension>
where
	T: SysConfig + ContractConfig + pallet::Config,
	<<T as SysConfig>::Lookup as StaticLookup>::Source: From<<T as SysConfig>::AccountId>,
	E: Ext<T = T>,
	<E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
{
	fn test_method(&mut self, val: u64) -> u32 {
		Pallet::<T>::test_method(RawOrigin::Signed(self.env.ext().address().clone()).into(), val)
			.unwrap();
		123456
	}
}
