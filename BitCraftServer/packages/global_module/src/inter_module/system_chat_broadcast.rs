use spacetimedb::{ReducerContext, Table};

use crate::{
    game::game_state::{create_entity, unix},
    messages::{
        global::{direct_message_state, DirectMessageState},
        inter_module::AdminBroadcastMessageMsg,
    },
    signed_in_player_state,
};

pub fn process_message_on_destination(ctx: &ReducerContext, request: AdminBroadcastMessageMsg) -> Result<(), String> {
    for player in ctx.db.signed_in_player_state().iter() {
        ctx.db.direct_message_state().try_insert(DirectMessageState {
            entity_id: create_entity(ctx),
            username: "Wisp".into(),
            title_id: 0,
            sender_entity_id: 0,
            receiver_entity_id: player.entity_id,
            text: request.message.clone(),
            timestamp: unix(ctx.timestamp),
            language_code: None,
        })?;
    }

    Ok(())
}
