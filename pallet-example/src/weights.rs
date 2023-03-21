use frame_support::weights::Weight;

pub trait WeightInfo {
	fn const_ref_time(val: u64) -> Weight;
}

impl<I> WeightInfo for I {
	fn const_ref_time(val: u64) -> Weight {
		Weight::from_parts(val, 0)
	}
}
