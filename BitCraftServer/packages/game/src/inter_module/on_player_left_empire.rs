use spacetimedb::ReducerContext;

use crate::messages::{empire_shared::EmpireState, inter_module::OnPlayerLeftEmpireMsg};

pub fn process_message_on_destination(ctx: &ReducerContext, request: OnPlayerLeftEmpireMsg) -> Result<(), String> {
    EmpireState::unequip_cloak(ctx, request.player_entity_id);
    Ok(())
}
