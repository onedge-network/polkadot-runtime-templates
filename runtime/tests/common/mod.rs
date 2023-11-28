// ExtBuilder impl for all runtime integration tests
use frame_support::weights::Weight;
use parachain_template_runtime::{BuildStorage, Runtime, System};

pub fn run_with_system_weight<F>(w: Weight, mut assertions: F)
where
    F: FnMut() -> (),
{
    let mut t: sp_io::TestExternalities =
        frame_system::GenesisConfig::<Runtime>::default().build_storage().unwrap().into();
    t.execute_with(|| {
        System::set_block_consumed_resources(w, 0);
        assertions()
    });
}
