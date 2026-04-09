use spacetimedb::ReducerContext;

use crate::messages::{components::experience_state, inter_module::RestoreSkillsMsg};

pub fn process_message_on_destination(ctx: &ReducerContext, request: RestoreSkillsMsg) -> Result<(), String> {
    let mut experience = ctx.db.experience_state().entity_id().find(request.player_entity_id).unwrap();

    for stack in request.experience_stacks {
        if let Some(current_stack) = experience.experience_stacks.iter_mut().find(|es| es.skill_id == stack.skill_id) {
            current_stack.quantity = current_stack.quantity.max(stack.quantity);
        } else {
            experience.experience_stacks.push(stack);
        }
    }

    ctx.db.experience_state().entity_id().update(experience);

    Ok(())
}
