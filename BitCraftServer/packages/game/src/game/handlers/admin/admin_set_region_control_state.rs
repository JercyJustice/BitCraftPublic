use bitcraft_macro::shared_table_reducer;
use spacetimedb::ReducerContext;

use crate::{
    game::handlers::authentication::has_role,
    messages::{
        authentication::Role,
        generic::{globals, region_control_info, RegionControlInfo},
    },
    unwrap_or_err,
};

#[spacetimedb::reducer]
#[shared_table_reducer]
pub fn admin_set_region_control_state(ctx: &ReducerContext, allow_players: bool, allow_player_spawn: bool) -> Result<(), String> {
    if !has_role(ctx, &ctx.sender, Role::Admin) {
        return Err("Unauthorized".into());
    }

    let region_id = unwrap_or_err!(ctx.db.globals().version().find(0), "GLobals not found").region_index;
    let mut control = unwrap_or_err!(ctx.db.region_control_info().region_id().find(region_id), "Region not initialized");
    control.allow_players = allow_players;
    control.allow_player_spawns = allow_player_spawn;
    RegionControlInfo::update_shared(ctx, control, crate::inter_module::InterModuleDestination::GlobalAndAllOtherRegions);

    Ok(())
}
