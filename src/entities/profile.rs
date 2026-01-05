use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

use super::{CertificatesType, DeviceType, PagingInformation, SelfAndRelatedLinks, SelfLinks};

query_params!(ProfileQuery{
    fields_certificates("fields[certificates]",String),
    fields_devices("fields[devices]",String),
    filter_profiles("filter[profiles]",String),
    filter_id("filter[id]",String),
    filter_name("filter[name]",String),
    include("include",String),
    limit("limit", i64),
    limit_certificates("limit[certificates]",i64),
    limit_devices("limit[devices]",i64),
    sort("sort",ProfileSort),
    fields_bundle_ids("fields[bundleIds]",String),
    filter_profile_state(" filter[profileState]",ProfileState),
    filter_profile_type("filter[profileType]",ProfileType),
});

enum_str!(ProfileSort{
    Id("id"),
    IdDesc("-id"),
    Name("name"),
    NameDesc("-name"),
    ProfileState("profileState"),
    ProfileStateDesc("-profileState"),
    ProfileType("profileType"),
    ProfileTypeDesc("-profileType"),
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    #[serde(rename = "type")]
    pub type_field: ProfilesType,
    pub id: String,
    pub attributes: ProfileAttributes,
    pub relationships: ProfileRelationships,
    pub links: SelfLinks,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileAttributes {
    #[serde(rename = "profileState")]
    pub profile_state: ProfileState,
    #[serde(rename = "createdDate")]
    pub created_date: DateTime<Utc>,
    #[serde(rename = "profileType")]
    pub profile_type: ProfileType,
    pub name: String,
    #[serde(rename = "profileContent")]
    pub profile_content: String,
    pub uuid: String,
    pub platform: String,
    #[serde(rename = "expirationDate")]
    pub expiration_date: DateTime<Utc>,
}

enum_str!(ProfileState{
    INVALID("INVALID"),
    ACTIVE("ACTIVE"),
});

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileRelationships {
    #[serde(rename = "bundleId")]
    pub bundle_id: BundleIdMeta,
    pub certificates: Certificates,
    pub devices: Devices,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BundleIdMeta {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Certificates {
    pub meta: PagingInformation,
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Devices {
    pub meta: PagingInformation,
    pub links: SelfAndRelatedLinks,
}

enum_str!(ProfilesType{
    Profiles("profiles"),
});

enum_str!(ProfileType
 {
    IosAppDevelopment("IOS_APP_DEVELOPMENT"),
    IosAppStore("IOS_APP_STORE"),
    IosAppAdhoc("IOS_APP_ADHOC"),
    IosAppInhouse("IOS_APP_INHOUSE"),
    MacAppDevelopment("MAC_APP_DEVELOPMENT"),
    MacAppStore("MAC_APP_STORE"),
    MacAppDirect("MAC_APP_DIRECT"),
    TvosAppDevelopment("TVOS_APP_DEVELOPMENT"),
    TvosAppStore("TVOS_APP_STORE"),
    TvosAppAdhoc("TVOS_APP_ADHOC"),
    TvosAppInhouse("TVOS_APP_INHOUSE"),
    MacCatalystAppDevelopment("MAC_CATALYST_APP_DEVELOPMENT"),
    MacCatalystAppStore("MAC_CATALYST_APP_STORE"),
    MacCatalystAppDirect("MAC_CATALYST_APP_DIRECT"),
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileCreateRequest {
    pub data: ProfileCreateRequestData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileCreateRequestData {
    pub attributes: ProfileCreateRequestAttributes,
    pub relationships: ProfileCreateRequestRelationships,
    #[serde(rename = "type")]
    pub type_field: ProfileCreateRequestType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileCreateRequestAttributes {
    pub name: String,
    #[serde(rename = "profileType")]
    pub profile_type: ProfileType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileCreateRequestRelationships {
    #[serde(rename = "bundleId")]
    pub bundle_id: ProfileCreateRequestDataRelationshipsBundleId,
    pub certificates: ProfileCreateRequestDataRelationshipsCertificates,
    pub devices: Option<ProfileCreateRequestDataRelationshipsDevices>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileCreateRequestDataRelationshipsBundleId {
    pub data: ProfileCreateRequestDataRelationshipsBundleIdData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileCreateRequestDataRelationshipsBundleIdData {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: super::BundleIdsType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileCreateRequestDataRelationshipsCertificates {
    pub data: Vec<ProfileCreateRequestDataRelationshipsCertificatesData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileCreateRequestDataRelationshipsCertificatesData {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: CertificatesType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileCreateRequestDataRelationshipsDevices {
    pub data: Vec<ProfileCreateRequestDataRelationshipsDevicesData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileCreateRequestDataRelationshipsDevicesData {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: DeviceType,
}

enum_str!(ProfileCreateRequestType{
   Profiles("profiles"),
});

