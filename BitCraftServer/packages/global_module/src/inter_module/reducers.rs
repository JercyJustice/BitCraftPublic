use bitcraft_macro::shared_table_reducer;
use spacetimedb::{ReducerContext, Table};

use crate::{
    game::handlers::authentication::has_role,
    messages::{
        authentication::{Role, ServerIdentity},
        inter_module::{
            inter_module_message, inter_module_message_counter, inter_module_message_errors, InterModuleMessage, InterModuleMessageCounter,
            InterModuleMessageErrors, MessageContents,
        },
    },
};

use super::*;

//Called on destination module
#[spacetimedb::reducer]
#[shared_table_reducer]
pub fn process_inter_module_message(ctx: &ReducerContext, sender: u8, message: InterModuleMessage) -> Result<(), String> {
    validate_relay_identity(ctx)?;

    if let Some(mut counter) = ctx.db.inter_module_message_counter().module_id().find(&sender) {
        if counter.last_processed_message_id >= message.id {
            //Message was already processed
            spacetimedb::log::warn!("Inter-module message {} was already processed", message.id);
            if let Some(r) = ctx.db.inter_module_message_errors().id().filter((sender, message.id)).next() {
                return Err(r.error);
            }
            return Ok(());
        }
        counter.last_processed_message_id = message.id;
        ctx.db.inter_module_message_counter().module_id().update(counter);
    } else {
        ctx.db.inter_module_message_counter().insert(InterModuleMessageCounter {
            module_id: sender,
            last_processed_message_id: message.id,
        });
    }

    let r = match message.contents {
        MessageContents::TableUpdate(u) => {
            u.apply_updates(ctx);
            Ok(())
        }

        MessageContents::TransferPlayerRequest(_) => panic!("Global module should never receive TransferPlayerRequest message"),
        MessageContents::TransferPlayerHousingRequest(_) => {
            panic!("Global module should never receive TransferPlayerHousingRequest message")
        }
        MessageContents::PlayerCreateRequest(_) => panic!("Global module should never receive PlayerCreateRequest message"),
        MessageContents::OnPlayerNameSetRequest(_) => panic!("Global module should never receive OnPlayerNameSetRequest message"),
        MessageContents::OnEmpireBuildingDeleted(_) => panic!("Global module should never receive OnEmpireBuildingDeleted message"),
        MessageContents::OnPlayerJoinedEmpire(_) => panic!("Global module should never receive OnPlayerJoinedEmpire message"),
        MessageContents::OnPlayerLeftEmpire(_) => panic!("Global module should never receive OnPlayerLeftEmpire message"),
        MessageContents::RegionDestroySiegeEngine(_) => panic!("Global module should never receive RegionDestroySiegeEngine message"),
        MessageContents::EmpireUpdateEmperorCrown(_) => panic!("Global module should never receive EmpireUpdateEmperorCrown message"),
        MessageContents::EmpireRemoveCrown(_) => panic!("Global module should never receive EmpireRemoveCrown message"),
        MessageContents::SignPlayerOut(_) => panic!("Global module should never receive SignPlayerOut message"),
        MessageContents::AdminBroadcastMessage(_) => panic!("Global module should never receive AdminBroadcastMessage message"),
        MessageContents::PlayerSkipQueue(_) => panic!("Global module should never receive PlayerSkipQueue message"),
        MessageContents::GrantHubItem(_) => panic!("Global module should never receive GrantHubItem message"),
        MessageContents::RecoverDeployable(_) => panic!("Global module should never receive RecoverDeployable message"),
        MessageContents::OnDeployableRecovered(_) => panic!("Global module should never receive OnDeployableRecovered message"),
        MessageContents::ReplaceIdentity(_) => panic!("Global module should never receive ReplaceIdentity message"),
        MessageContents::RestoreSkills(_) => panic!("Global module should never receive RestoreSkills message"),

        MessageContents::UserUpdateRegionRequest(r) => user_update_region::process_message_on_destination(ctx, r),
        MessageContents::ClaimCreateEmpireSettlementState(r) => {
            claim_create_empire_settlement_state::process_message_on_destination(ctx, r)
        }
        MessageContents::OnClaimMembersChanged(r) => on_claim_members_changed::process_message_on_destination(ctx, r),
        MessageContents::EmpireCreateBuilding(r) => empire_create_building::process_message_on_destination(ctx, r),
        MessageContents::GlobalDeleteEmpireBuilding(r) => global_delete_empire_building::process_message_on_destination(ctx, r),
        MessageContents::DeleteEmpire(r) => delete_empire::process_message_on_destination(ctx, r),
        MessageContents::EmpireClaimJoin(r) => empire_claim_join::process_message_on_destination(ctx, r),
        MessageContents::EmpireResupplyNode(r) => empire_resupply_node::process_message_on_destination(ctx, r),
        MessageContents::EmpireDonateItem(r) => empire_donate_item::process_message_on_destination(ctx, r),
        MessageContents::EmpireCreate(r) => empire_create::process_message_on_destination(ctx, r),
        MessageContents::EmpireCollectHexiteCapsule(r) => empire_collect_hexite_capsule::process_message_on_destination(ctx, r),
        MessageContents::EmpireStartSiege(r) => empire_start_siege::process_message_on_destination(ctx, r),
        MessageContents::EmpireSiegeAddSupplies(r) => empire_siege_add_supplies::process_message_on_destination(ctx, r),
        MessageContents::EmpireAddCurrency(r) => empire_add_currency::process_message_on_destination(ctx, r),
        MessageContents::OnRegionPlayerCreated(r) => on_region_player_created::process_message_on_destination(ctx, r),
        MessageContents::EmpireQueueSupplies(r) => empire_queue_supplies::process_message_on_destination(ctx, r),
        MessageContents::ClaimSetName(r) => claim_set_name::process_message_on_destination(ctx, r),
        MessageContents::NpcPlaceWatchtowers(r) => npc_place_watchtowers::process_message_on_destination(ctx, r),
    };

    if let Err(error) = r.clone() {
        spacetimedb::volatile_nonatomic_schedule_immediate!(save_inter_module_message_error(sender, message.id, error));
    }

    return r;
}

#[spacetimedb::reducer()]
fn save_inter_module_message_error(ctx: &ReducerContext, sender: u8, message_id: u64, error: String) {
    if let Err(_) = ServerIdentity::validate_server_only(ctx) {
        return;
    }
    ctx.db.inter_module_message_errors().insert(InterModuleMessageErrors {
        sender_module_id: sender,
        message_id: message_id,
        error: error,
    });
}

//Called on sender module
#[spacetimedb::reducer]
pub fn on_inter_module_message_processed(ctx: &ReducerContext, id: u64, error: Option<String>) -> Result<(), String> {
    validate_relay_identity(ctx)?;

    if let Some(err) = &error {
        spacetimedb::log::error!("Inter-module reducer {id} returned error: {err}");
    }

    let message = ctx.db.inter_module_message().id().find(id).unwrap();
    match message.contents {
        MessageContents::PlayerCreateRequest(r) => player_create::handle_destination_result_on_sender(ctx, r, error),
        MessageContents::GrantHubItem(r) => grant_hub_item::handle_destination_result_on_sender(ctx, r, error),
        _ => {}
    }

    ctx.db.inter_module_message().id().delete(id);
    return Ok(());
}

fn validate_relay_identity(ctx: &ReducerContext) -> Result<(), String> {
    if !has_role(ctx, &ctx.sender, Role::Admin) {
        return Err("Unauthorized".into());
    }
    return Ok(());
}
