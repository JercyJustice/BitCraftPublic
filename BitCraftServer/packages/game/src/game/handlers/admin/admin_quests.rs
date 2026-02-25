use spacetimedb::{log, ReducerContext, Table};

use crate::{
    game::handlers::authentication::has_role,
    messages::{
        authentication::Role,
        components::quest_chain_state,
    },
};

// Deletes everyone's quest progress towards a certain quest if they haven't completed it yet.
// This is useful for seasonal quests that are no longer completable because a seasonal resource no longer exists.
// If we don't delete the quest progress, it'll sit in their quest log forever.
#[spacetimedb::reducer]
pub fn admin_fail_quest(ctx: &ReducerContext, quest_desc_id: i32) {
    if !has_role(ctx, &ctx.sender, Role::Admin) {
        log::error!("Unauthorized.");
        return;
    }

    for quest_state in ctx.db.quest_chain_state().iter().filter(|qcs| qcs.quest_chain_desc_id == quest_desc_id) {
        if !quest_state.completed {
            ctx.db.quest_chain_state().delete(quest_state);
        }
    }
}
