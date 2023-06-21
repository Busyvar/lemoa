use lemmy_api_common::{community::{GetCommunity, GetCommunityResponse, CommunityResponse, FollowCommunity}, lemmy_db_schema::newtypes::CommunityId};

use crate::settings;

pub fn get_community(name: String) -> std::result::Result<GetCommunityResponse, reqwest::Error> {
    let params = GetCommunity {
        name: Some(name),
        auth: settings::get_current_account().jwt,
        ..Default::default()
    };

    super::get("/community", &params)
}

pub fn follow_community(
    community_id: i32,
    follow: bool,
) -> Result<CommunityResponse, reqwest::Error> {
    let params = FollowCommunity {
        community_id: CommunityId(community_id),
        follow,
        auth: settings::get_current_account().jwt.unwrap(),
    };
    super::post("/community/follow", &params)
}

pub fn default_community() -> GetCommunityResponse {
    serde_json::from_str(include_str!("../examples/community.json")).unwrap()
}
