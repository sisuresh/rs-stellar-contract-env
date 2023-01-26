use soroban_env_common::{
    xdr::{
        ContractEvent, ContractEventBody, ContractEventType, ContractEventV0, ExtensionPoint,
        ScVal, ScVec,
    },
    ConversionError, Object, Symbol, TryFromVal,
};

use crate::{budget::AsBudget, Host, HostError, RawVal};

/// None of these functions are metered, which is why they're behind the is_debug check
impl Host {
    fn record_system_debug_contract_event(
        &self,
        type_: ContractEventType,
        topics: ScVec,
        data: ScVal,
    ) -> Result<(), HostError> {
        if !self.is_debug() {
            return Ok(());
        }

        let ce = ContractEvent {
            ext: ExtensionPoint::V0,
            contract_id: None,
            type_,
            body: ContractEventBody::V0(ContractEventV0 { topics, data }),
        };
        self.get_events_mut(|events| Ok(events.record_structured_debug_event(ce)))?;
        Ok(())
    }

    // Emits an event with topic = ["fn_call", contract_id, function_name] and
    // data = [arg1, args2, ...]
    pub fn fn_call_diagnostics(
        &self,
        contract_id: Object,
        func: Symbol,
        args: &[RawVal],
    ) -> Result<(), HostError> {
        if !self.is_debug() {
            return Ok(());
        }

        self.as_budget().with_free_budget(|| {
            let mut topics: Vec<ScVal> = Vec::new();
            topics.push(ScVal::Symbol(
                self.map_err("fn_call".as_bytes().try_into())?,
            ));

            let id = ScVal::Object(Some(self.from_host_obj(contract_id)?));

            topics.push(id);
            topics.push(func.try_into()?);

            let data: Result<Vec<ScVal>, ConversionError> = args
                .into_iter()
                .map(|i| ScVal::try_from_val(self, i))
                .collect();
            let data_vec: ScVec = self.map_err(data?.try_into())?;

            self.record_system_debug_contract_event(
                ContractEventType::System,
                self.map_err(topics.try_into())?,
                data_vec.try_into()?,
            )
        })
    }

    // Emits an event with topic = ["fn_return", contract_id, function_name] and
    // data = [return_val]
    pub fn fn_return_diagnostics(
        &self,
        contract_id: Object,
        func: Symbol,
        res: RawVal,
    ) -> Result<(), HostError> {
        if !self.is_debug() {
            return Ok(());
        }

        self.as_budget().with_free_budget(|| {
            let mut topics: Vec<ScVal> = Vec::new();
            topics.push(ScVal::Symbol(
                self.map_err("fn_return".as_bytes().try_into())?,
            ));

            let id = ScVal::Object(Some(self.from_host_obj(contract_id)?));

            topics.push(id);
            topics.push(func.try_into()?);

            let return_val = ScVal::try_from_val(self, &res)?;

            self.record_system_debug_contract_event(
                ContractEventType::System,
                self.map_err(topics.try_into())?,
                return_val,
            )
        })
    }
}
