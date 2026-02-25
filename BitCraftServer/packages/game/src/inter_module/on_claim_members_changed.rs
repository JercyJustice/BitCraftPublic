use spacetimedb::ReducerContext;

use crate::messages::inter_module::{MessageContents, OnClaimMembersChangedMsg};

use super::send_inter_module_message;

pub fn send_message(ctx: &ReducerContext, claim_entity_id: u64) {
    send_inter_module_message(
        ctx,
        MessageContents::OnClaimMembersChanged(OnClaimMembersChangedMsg { claim_entity_id }),
        super::InterModuleDestination::Global,
    );
}
