use spacetimedb::ReducerContext;

use crate::{
    messages::{empire_shared::*, inter_module::*},
    unwrap_or_err,
};

pub fn process_message_on_destination(ctx: &ReducerContext, request: EmpireAddCurrencyMsg) -> Result<(), String> {
    let mut empire = unwrap_or_err!(
        ctx.db.empire_state().entity_id().find(request.empire_entity_id),
        "This empire does not exist"
    );
    empire.empire_currency_treasury += request.amount;
    EmpireState::update_shared(ctx, empire, super::InterModuleDestination::AllOtherRegions);

    Ok(())
}
