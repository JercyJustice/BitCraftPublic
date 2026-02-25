use crate::messages::authentication::Role;
use crate::messages::components::previous_player_skills_state;
use crate::messages::game_util::ExperienceStack;
use crate::{game::handlers::authentication::has_role, messages::components::PreviousPlayerSkillsState};
use spacetimedb::ReducerContext;

#[spacetimedb::reducer]
pub fn admin_populate_player_skills(ctx: &ReducerContext, identity_skill_stacks_row: String) -> Result<(), String> {
    // To use this, do the following:
    // spacetime sql "SELECT user_state.identity, experience_state.experience_stacks FROM experience_state JOIN user_state ON user_state.entity_id=experience_state.entity_id"
    // This will return rows under the form of:
    //  0x13679cea332c56f468f6e79b9ed9e8a4d01999ced1ced3e1f0c6ecc9f92f2dfe | (skill_id = 1, quantity = 0)(skill_id = 3, quantity = 0)(skill_id = 15, quantity = 0)(skill_id = 13, quantity = 0)(skill_id = 11, quantity = 0)(skill_id = 12, quantity = 0)(skill_id = 14, quantity = 0)(skill_id = 2, quantity = 0)(skill_id = 22, quantity = 0)(skill_id = 9, quantity = 0)(skill_id = 8, quantity = 0)(skill_id = 4, quantity = 0)(skill_id = 19, quantity = 0)(skill_id = 5, quantity = 0)(skill_id = 21, quantity = 0)(skill_id = 7, quantity = 0)(skill_id = 18, quantity = 0)(skill_id = 6, quantity = 0)(skill_id = 10, quantity = 0)(skill_id = 17, quantity = 0)
    // use each of these rows for the 'identity_skill_stacks_row'
    if !has_role(ctx, &ctx.sender, Role::Admin) {
        return Err("Invalid permissions".into());
    }

    let i = identity_skill_stacks_row.find('|').unwrap();
    let identity_str = identity_skill_stacks_row.as_str()[0..i].trim().to_string();

    let identity = match identity_str.parse() {
        Ok(i) => i,
        Err(_) => return Err("Failed to parse identity".into()),
    };

    let mut experience_stacks = Vec::new();
    let mut j = i + 1;
    while j < identity_skill_stacks_row.len() {
        let skill_stacks = &identity_skill_stacks_row.as_str()[j..];
        let a = skill_stacks.find("id = ").unwrap();
        let b = skill_stacks.find(", quantity = ").unwrap();
        let c = skill_stacks.find(")").unwrap();
        let skill_id_str = &skill_stacks[a + 5..b];
        let quantity_str = &skill_stacks[b + 13..c];
        let exp_stack = ExperienceStack {
            skill_id: skill_id_str.parse().ok().unwrap(),
            quantity: quantity_str.parse().ok().unwrap(),
        };
        experience_stacks.push(exp_stack);
        j += c + 1;
    }
    ctx.db
        .previous_player_skills_state()
        .identity()
        .insert_or_update(PreviousPlayerSkillsState {
            identity,
            experience_stacks,
        });

    Ok(())
}
