use spacetimedb::ReducerContext;

use crate::messages::{
    components::{claim_lowercase_name_state, ClaimLowercaseNameState},
    inter_module::ClaimSetNameMsg,
};

pub fn process_message_on_destination(ctx: &ReducerContext, request: ClaimSetNameMsg) -> Result<(), String> {
    let name_lowercase = request.new_name.to_lowercase();
    if name_lowercase.starts_with("claimed area (") {
        return Err("This name cannot be used".into());
    }

    if let Some(c) = ctx.db.claim_lowercase_name_state().name_lowercase().find(&name_lowercase) {
        if c.entity_id != request.claim_entity_id {
            //Allow claims to change capitalization of their name
            return Err("This name is already taken".into());
        }
    }

    if let Some(mut c) = ctx.db.claim_lowercase_name_state().entity_id().find(request.claim_entity_id) {
        c.name_lowercase = name_lowercase;
        ClaimLowercaseNameState::update_shared(ctx, c, super::InterModuleDestination::AllOtherRegions);
    } else {
        ClaimLowercaseNameState::insert_shared(
            ctx,
            ClaimLowercaseNameState {
                entity_id: request.claim_entity_id,
                name_lowercase,
            },
            super::InterModuleDestination::AllOtherRegions,
        );
    }

    Ok(())
}
