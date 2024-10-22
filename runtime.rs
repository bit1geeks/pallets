// Add this at the top with other imports
pub use kv_store::{self as kv_store_pallet};

// Inside the runtime configuration
impl pallet::Config for Runtime {}

// Add to the construct_runtime! macro
kv_store: kv_store_pallet::{Pallet::<Runtime>, Config},
