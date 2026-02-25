use spacetimedb::{Identity, ReducerContext};

use crate::{
    messages::{
        components::{user_state, vault_state},
        inter_module::GrantHubItemMsg,
        static_data::premium_item_desc,
    },
    unwrap_or_err,
};

pub fn process_message_on_destination(ctx: &ReducerContext, request: GrantHubItemMsg) -> Result<(), String> {
    match request.item_type {
        crate::messages::generic::HubItemType::Collectible => {
            grant_collectible(ctx, request.player_identity, request.item_id, request.quantity)?;
            Ok(())
        }
        crate::messages::generic::HubItemType::PremiumItem => {
            let premium_item = ctx.db.premium_item_desc().id().find(request.item_id).unwrap();
            for collectible_id in premium_item.collectible_ids {
                grant_collectible(ctx, request.player_identity, collectible_id, request.quantity)?;
            }
            Ok(())
        }
        _ => panic!("HubItemType {:?} is unhandled", request.item_type),
    }
}

fn grant_collectible(ctx: &ReducerContext, player_identity: Identity, collectible_id: i32, quantity: u32) -> Result<(), String> {
    let user_state = unwrap_or_err!(ctx.db.user_state().identity().find(player_identity), "Unknown user");
    let mut vault_state = unwrap_or_err!(ctx.db.vault_state().entity_id().find(user_state.entity_id), "Unknown vault state");

    for _i in 0..quantity {
        // When we grant hub items, we want to add locked collectibles even if already present
        vault_state.add_collectible(ctx, collectible_id, true)?;
    }

    ctx.db.vault_state().entity_id().update(vault_state);

    Ok(())
}
