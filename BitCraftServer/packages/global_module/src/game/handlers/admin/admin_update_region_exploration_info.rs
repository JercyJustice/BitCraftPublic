use bitcraft_macro::shared_table_reducer;
use spacetimedb::ReducerContext;

use crate::{
    game::handlers::authentication::has_role,
    inter_module::InterModuleDestination,
    messages::{
        authentication::Role,
        generic::{region_exploration_info, world_region_state, RegionExplorationInfo},
    },
    unwrap_or_err,
};

#[shared_table_reducer]
#[spacetimedb::reducer]
pub fn admin_update_region_exploration_info(ctx: &ReducerContext, region_id: u8, counts_toward_achievements: bool) -> Result<(), String> {
    if !has_role(ctx, &ctx.sender, Role::Admin) {
        return Err("Unauthorized".into());
    }

    let world_region_state = unwrap_or_err!(ctx.db.world_region_state().id().find(0), "Failed to get WorldRegionState");
    if region_id == 0 || region_id > world_region_state.region_count {
        return Err(format!("Region id {region_id} is out of range"));
    }

    let info = RegionExplorationInfo {
        region_id,
        counts_toward_achievements,
    };

    if ctx.db.region_exploration_info().region_id().find(region_id).is_some() {
        RegionExplorationInfo::update_shared(ctx, info, InterModuleDestination::AllOtherRegions);
    } else {
        RegionExplorationInfo::insert_shared(ctx, info, InterModuleDestination::AllOtherRegions);
    }

    Ok(())
}
