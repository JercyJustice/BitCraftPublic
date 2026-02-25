use bitcraft_macro::shared_table_reducer;
use spacetimedb::ReducerContext;

use crate::{
    game::handlers::authentication::has_role,
    inter_module::InterModuleDestination,
    messages::{
        authentication::Role,
        empire_shared::{empire_lowercase_name_state, empire_state, EmpireState},
    },
    unwrap_or_err,
};

#[spacetimedb::reducer]
pub fn admin_rename_empire(ctx: &ReducerContext, current_name: String, new_name: String) -> Result<(), String> {
    if !has_role(ctx, &ctx.sender, Role::Gm) {
        return Err("Unauthorized".into());
    }

    let name_lower = current_name.to_lowercase();
    let entity_id = unwrap_or_err!(
        ctx.db.empire_lowercase_name_state().name_lowercase().find(name_lower),
        "Empire not found"
    )
    .entity_id;

    admin_rename_empire_entity(ctx, entity_id, new_name)
}

#[shared_table_reducer]
#[spacetimedb::reducer]
pub fn admin_rename_empire_entity(ctx: &ReducerContext, entity_id: u64, new_name: String) -> Result<(), String> {
    if !has_role(ctx, &ctx.sender, Role::Gm) {
        return Err("Unauthorized".into());
    }

    if ctx.db.empire_state().name().find(&new_name).is_some() {
        return Err("An empire with this name already exists".into());
    }

    let mut empire = unwrap_or_err!(ctx.db.empire_state().entity_id().find(entity_id), "Empire not found");
    empire.name = new_name.clone();
    EmpireState::update_shared(ctx, empire, InterModuleDestination::AllOtherRegions);

    let mut empire_lower = unwrap_or_err!(
        ctx.db.empire_lowercase_name_state().entity_id().find(entity_id),
        "Empire lowercase name not found"
    );
    empire_lower.name_lowercase = new_name.to_lowercase();
    ctx.db.empire_lowercase_name_state().entity_id().update(empire_lower);

    Ok(())
}
