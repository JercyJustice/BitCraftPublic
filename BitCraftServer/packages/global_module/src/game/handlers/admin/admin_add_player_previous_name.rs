use crate::{
    game::handlers::authentication::has_role,
    messages::{
        authentication::Role,
        components::{previous_player_username_state, PreviousPlayerUsernameState},
    },
};
use spacetimedb::{Identity, ReducerContext, Table};
use std::str::FromStr;

#[spacetimedb::reducer]
pub fn admin_add_player_previous_name(ctx: &ReducerContext, identity: String, name: String) -> Result<(), String> {
    if !has_role(ctx, &ctx.sender, Role::Admin) {
        return Err("Unauthorized".into());
    }

    let identity = Identity::from_str(identity.as_str());
    if identity.is_err() {
        return Err("Identity couldn't be parsed".into());
    }
    let identity = identity.unwrap();

    ctx.db.previous_player_username_state().insert(PreviousPlayerUsernameState {
        identity,
        name: name.clone(),
        lower_case_name: name.to_lowercase(),
    });

    Ok(())
}
