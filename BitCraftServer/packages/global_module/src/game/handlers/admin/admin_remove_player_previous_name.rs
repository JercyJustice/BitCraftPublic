use crate::{
    game::handlers::authentication::has_role,
    messages::{authentication::Role, components::previous_player_username_state},
};
use spacetimedb::ReducerContext;

#[spacetimedb::reducer]
pub fn admin_remove_player_previous_name(ctx: &ReducerContext, name: String) -> Result<(), String> {
    if !has_role(ctx, &ctx.sender, Role::Admin) {
        return Err("Unauthorized".into());
    }

    ctx.db
        .previous_player_username_state()
        .lower_case_name()
        .delete(name.to_lowercase());

    Ok(())
}
