use frame_support::weights::Weight;

pub trait WeightInfo {
	fn test_method(val: u64) -> Weight;
}

impl<I> WeightInfo for I {
	fn test_method(val: u64) -> Weight {
		Weight::from_ref_time(val)
	}
}
