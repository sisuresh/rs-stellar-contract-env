use soroban_env_common::Env;

use crate::{
    budget::Budget,
    host::metered_map::MeteredOrdMap,
    storage::{Footprint, Storage},
    Host, HostError, LedgerInfo,
};

#[test]
fn ledger_network_id() -> Result<(), HostError> {
    let budget = Budget::default();
    let storage = Storage::with_enforcing_footprint_and_map(
        Footprint::default(),
        MeteredOrdMap::new(&budget)?,
    );

    let host = Host::with_storage_and_budget(storage, budget.clone());
    host.set_ledger_info(LedgerInfo {
        protocol_version: 0,
        sequence_number: 0,
        timestamp: 0,
        network_id: [7; 32],
        base_reserve: 0,
    });
    let obj = host.get_ledger_network_id()?;
    let np = host.visit_obj(obj, |np: &Vec<u8>| Ok(np.clone()))?;
    assert_eq!(np, vec![7; 32],);
    Ok(())
}
