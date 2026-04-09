// ============================================================================
// Region Moderation Config Table
// ============================================================================

/// Per-region chat rate-limit and new-account restriction settings.
/// Singleton table (id = 0).
#[spacetimedb::table(name = region_moderation_config_state)]
#[derive(Clone, Debug)]
pub struct RegionModerationConfigState {
    #[primary_key]
    pub id: u8, // always 0 (singleton)
    pub max_messages_per_time_period: u32,
    pub rate_limit_window_sec: i32,
    pub new_account_min_playtime_sec: i32,
}

impl RegionModerationConfigState {
    pub const DEFAULT_MAX_MESSAGES_PER_TIME_PERIOD: u32 = 3;
    pub const DEFAULT_RATE_LIMIT_WINDOW_SEC: i32 = 15;
    pub const DEFAULT_NEW_ACCOUNT_MIN_PLAYTIME_SEC: i32 = 0;
}
