use obce::ink_lang::env::DefaultEnvironment;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct Extension;

impl crate::ChainExtension<DefaultEnvironment> for Extension {}
