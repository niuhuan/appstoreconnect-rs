use serde_derive::{Deserialize, Serialize};

use super::AppsType;

query_params!(UsersQuery {
    fields_apps("fields[apps]",String),
    fields_users("fields[users]",String),
    include("include",String),
    limit("limit",i64),
    sort("sort",UserSort),
    filter_roles("filter[roles]",Role),
    filter_visible_apps("filter[visibleApps]",String),
    filter_username("filter[username]",String),
    limit_visible_apps("limit[visibleApps]",i64),
});

query_params!(UserVisibleAppsQuery {
    limit("limit",i64),
    fields_apps("fields[apps]",String),
});

enum_str!(UserSort{
    LastName("lastName"),
    LastNameDesc("-lastName"),
    Username("username"),
    UsernameDesc("-username"),
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "type")]
    pub type_field: UserType,
    pub id: String,
    pub attributes: UserAttributes,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserAttributes {
    pub username: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub roles: Vec<Role>,
    #[serde(rename = "allAppsVisible")]
    pub all_apps_visible: bool,
    #[serde(rename = "provisioningAllowed")]
    pub provisioning_allowed: bool,
}

enum_str!(UserType{
    Users("users"),
});

enum_str!(Role{
    Admin("ADMIN"),
    Finance("FINANCE"),
    AccountHolder("ACCOUNT_HOLDER"),
    Sales("SALES"),
    Marketing("MARKETING"),
    AppManager("APP_MANAGER"),
    Developer("DEVELOPER"),
    AccessToReports("ACCESS_TO_REPORTS"),
    CustomerSupport("CUSTOMER_SUPPORT"),
    ImageManager("IMAGE_MANAGER"),
    CreateApps("CREATE_APPS"),
    CloudManagedDeveloperId("CLOUD_MANAGED_DEVELOPER_ID"),
    CloudManagedAppDistribution("CLOUD_MANAGED_APP_DISTRIBUTION"),
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserUpdateRequest {
    pub data: UserUpdateRequestData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserUpdateRequestData {
    #[serde(rename = "type")]
    pub type_field: UserType,
    pub id: String,
    pub attributes: UserUpdateRequestDataAttributes,
    pub relationships: UserUpdateRequestDataRelationships,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserUpdateRequestDataAttributes {
    pub roles: Vec<Role>,
    #[serde(rename = "allAppsVisible")]
    pub all_apps_visible: bool,
    #[serde(rename = "provisioningAllowed")]
    pub provisioning_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserUpdateRequestDataRelationships {
    #[serde(rename = "visibleApps")]
    pub visible_apps: UserUpdateRequestDataRelationshipsVisibleApps,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserUpdateRequestDataRelationshipsVisibleApps {
    pub data: Vec<UserUpdateRequestDataRelationshipsVisibleAppsData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserUpdateRequestDataRelationshipsVisibleAppsData {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: AppsType,
}
