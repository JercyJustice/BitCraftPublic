use spacetimedb::{log, ReducerContext, Table};

use crate::{
    game::handlers::authentication::has_role,
    messages::{authentication::Role, components::*},
};

#[spacetimedb::reducer]
pub fn migrate_player_settings(ctx: &ReducerContext) -> Result<(), String> {
    if !has_role(ctx, &ctx.sender, Role::Admin) {
        return Err("Unauthorized".into());
    }

    let mut count = 0;

    for player_settings_state in ctx.db.player_settings_state().iter() {
        ctx.db.player_settings_state().insert(PlayerSettingsState {
            entity_id: player_settings_state.entity_id,
            fill_player_inventory: true,
            fill_deployable_inventory_first: player_settings_state.fill_deployable_inventory_first,
        });

        ctx.db.player_settings_state().delete(player_settings_state);
        count += 1;
    }

    log::info!("Migrated {} player settings", count);

    Ok(())
}
