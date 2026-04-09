use crate::game::handlers::authentication::has_role;
use crate::messages::authentication::Role;
use crate::messages::components::{previous_empire_name_state, PreviousEmpireNameState};
use spacetimedb::{log, ReducerContext};

#[spacetimedb::reducer]
pub fn admin_populate_empire_names(ctx: &ReducerContext, identity_empire_name_row: String) -> Result<(), String> {
    // To use this, do the following:
    // spacetime sql bitcraft-global "SELECT user_state.identity, empire_player_data_state.entity_id, empire_state.name FROM empire_player_data_state JOIN user_state ON user_state.entity_id=empire_player_data_state.entity_id JOIN empire_state ON empire_state.entity_id=empire_player_data_state.empire_entity_id WHERE empire_player_data_state.rank = 0"
    // This will return rows under the form of:
    //  0x13679cea332c56f468f6e79b9ed9e8a4d01999ced1ced3e1f0c6ecc9f92f2dfe | 12345678 | "My Empire"
    // use each of these rows for the 'identity_empire_name_row'
    if !has_role(ctx, &ctx.sender, Role::Admin) {
        return Err("Invalid permissions".into());
    }

    let i = identity_empire_name_row.find('|').unwrap();
    let identity_str = identity_empire_name_row.as_str()[0..i].trim().to_string();

    let emperor_identity = match identity_str.parse() {
        Ok(i) => i,
        Err(_) => return Err("Failed to parse identity".into()),
    };
    log::info!("emperor_identity = {emperor_identity}");

    // Skip entity id
    let left_over = identity_empire_name_row.as_str()[i + 1..].to_string();
    let i = left_over.find('|').unwrap();

    // Find empire name
    let empire_name = left_over.as_str()[i + 1..].trim().replace("\"", "").to_string();
    log::info!("empire_name = {empire_name}");

    ctx.db
        .previous_empire_name_state()
        .emperor_identity()
        .insert_or_update(PreviousEmpireNameState {
            emperor_identity,
            empire_name: empire_name.clone(),
            empire_lower_case_name: empire_name.to_lowercase(),
        });

    Ok(())
}
