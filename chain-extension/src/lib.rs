#![cfg_attr(not(feature = "std"), no_std)]

/// Implementation of the `PalletAssets` trait for the chain extension on `substrate` level.
#[cfg(feature = "substrate")]
pub mod substrate;

#[cfg(feature = "ink")]
pub mod ink;

#[obce::definition(id = "obce-chain-extension-example@v0.1")]
pub trait ChainExtension {
	fn test_method(&mut self, val: u64) -> u32;
}
