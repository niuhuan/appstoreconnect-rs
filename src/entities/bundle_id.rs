use serde_derive::{Deserialize, Serialize};

use super::{
    BundleIdPlatform, PagingInformation, PagedDocumentLinks, SelfAndRelatedLinks, SelfLinks,
};

query_params!(BundleIdQuery{
    fields_bundle_ids("fields[bundleIds]",String),
    fields_profiles("fields[profiles]",String),
    filter_id("filter[id]",String),
    filter_identifier("filter[identifier]",String),
    filter_name("filter[name]",String),
    filter_platform("filter[platform]", BundleIdPlatform),
    filter_seed_id("filter[seedId]", String),
    include("include",String),
    limit("limit", i64),
    limit_profiles("limit[profiles]",i64),
    sort("sort",BundleIdSort),
    fields_bundle_id_capabilities("fields[bundleIdCapabilities]",String),
    limit_bundle_id_capabilities("limit[bundleIdCapabilities]",i64),
    fields_apps("fields[apps]",String),
});

enum_str!(BundleIdSort {
    Id("id"),
    IdDesc("-id"),
    Identifier("identifier"),
    IdentifierDesc("-identifier"),
    Name("name"),
    NameDesc("-name"),
    Platform("platform"),
    PlatformDesc("-platform"),
    SeedIdType("seedId"),
    SeedIdDesc("-seedId"),
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BundleId {
    #[serde(rename = "type")]
    pub type_field: BundleIdsType,
    pub id: String,
    pub attributes: BundleIdAttributes,
    pub relationships: BundleIdRelationships,
    pub links: SelfLinks,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BundleIdAttributes {
    pub name: String,
    pub identifier: String,
    pub platform: String,
    #[serde(rename = "seedId")]
    pub seed_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BundleIdRelationships {
    #[serde(rename = "bundleIdCapabilities")]
    pub bundle_id_capabilities: BundleIdCapabilities,
    pub profiles: BundleIdProfiles,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BundleIdCapabilities {
    pub meta: PagingInformation,
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BundleIdProfiles {
    pub meta: PagingInformation,
    pub links: SelfAndRelatedLinks,
}

enum_str!(BundleIdsType{
    BundleIds("bundleIds"),
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BundleIdCreateRequest {
    pub data: BundleIdCreateRequestData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BundleIdCreateRequestData {
    #[serde(rename = "type")]
    pub type_field: BundleIdsType,
    pub attributes: BundleIdCreateRequestDataAttributes,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BundleIdCreateRequestDataAttributes {
    pub name: String,
    pub identifier: String,
    pub platform: BundleIdPlatform,
    #[serde(rename = "seedId")]
    pub seed_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BundleIdCapabilitiesWithoutIncludesResponse {
    pub data: Vec<BundleIdCapability>,
    pub links: PagedDocumentLinks,
    pub meta: PagingInformation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BundleIdCapability {
    #[serde(rename = "type")]
    pub type_field: BundleIdCapabilitiesType,
    pub id: String,
    pub attributes: BundleIdCapabilityAttributes,
    pub links: SelfLinks,
}

enum_str!(BundleIdCapabilitiesType{
    BundleIdCapabilities("bundleIdCapabilities"),
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BundleIdCapabilityAttributes {
    #[serde(rename = "capabilityType")]
    pub capability_type: CapabilityType,
    #[serde(rename = "settings")]
    pub settings: Option<serde_json::Value>,
}

enum_str!(CapabilityType{
    Icloud("ICLOUD"),
    InAppPurchase("IN_APP_PURCHASE"),
    GameCenter("GAME_CENTER"),
    PushNotifications("PUSH_NOTIFICATIONS"),
    Wallet("WALLET"),
    InterAppAudio("INTER_APP_AUDIO"),
    Maps("MAPS"),
    AssociatedDomains("ASSOCIATED_DOMAINS"),
    PersonalVpn("PERSONAL_VPN"),
    AppGroups("APP_GROUPS"),
    Healthkit("HEALTHKIT"),
    Homekit("HOMEKIT"),
    WirelessAccessoryConfiguration("WIRELESS_ACCESSORY_CONFIGURATION"),
    ApplePay("APPLE_PAY"),
    DataProtection("DATA_PROTECTION"),
    Sirikit("SIRIKIT"),
    NetworkExtensions("NETWORK_EXTENSIONS"),
    Multipath("MULTIPATH"),
    HotSpot("HOT_SPOT"),
    NfcTagReading("NFC_TAG_READING"),
    Classkit("CLASSKIT"),
    AutofillCredentialProvider("AUTOFILL_CREDENTIAL_PROVIDER"),
    AccessWifiInformation("ACCESS_WIFI_INFORMATION"),
    NetworkCustomProtocol("NETWORK_CUSTOM_PROTOCOL"),
    CoremediaHlsLowLatency("COREMEDIA_HLS_LOW_LATENCY"),
    SystemExtensionInstall("SYSTEM_EXTENSION_INSTALL"),
    UserManagement("USER_MANAGEMENT"),
    AppleIdAuth("APPLE_ID_AUTH"),
    UserNotificationsCommunication("USERNOTIFICATIONS_COMMUNICATION"),
    FamilyControls("FAMILY_CONTROLS"),
});
