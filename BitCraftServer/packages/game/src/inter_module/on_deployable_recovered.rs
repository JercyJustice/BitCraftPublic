use spacetimedb::{ReducerContext, Table};

use crate::{
    game::reducer_helpers::deployable_helpers,
    messages::{
        components::{deployable_state, trade_order_state, PlayerNotificationEvent},
        inter_module::OnDeployableRecoveredMsg,
        static_data::deployable_desc,
    },
    unwrap_or_err,
};

pub fn process_message_on_destination(ctx: &ReducerContext, msg: OnDeployableRecoveredMsg) -> Result<(), String> {
    let desc = unwrap_or_err!(
        ctx.db.deployable_desc().id().find(msg.deployable_desc_id),
        "DeployableDesc doesn't exist"
    );
    if let Err(err) = deployable_helpers::deactivate_deployable_collectible(ctx, msg.player_entity_id, &desc, false) {
        spacetimedb::log::error!("Failed to recover deployable: {}", err);
        PlayerNotificationEvent::new_event(
            ctx,
            msg.player_entity_id,
            err,
            crate::messages::components::NotificationSeverity::ReducerError,
        );
    }
    deployable_helpers::despawn(ctx, msg.deployable_entity_id);
    ctx.db.deployable_state().insert(msg.deployable_state);
    for t in msg.trade_orders {
        ctx.db.trade_order_state().insert(t);
    }
    Ok(())
}
