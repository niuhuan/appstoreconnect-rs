use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use serde_derive::Serialize;

// common

macro_rules! enum_str {
    ($name:ident { $($variant:ident($str:expr), )* }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $($variant,)*
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::Serializer,
            {
                serializer.serialize_str(match *self {
                    $( $name::$variant => $str, )*
                })
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(formatter, "a string for {}", stringify!($name))
                    }

                    fn visit_str<E>(self, value: &str) -> Result<$name, E>
                        where E: ::serde::de::Error,
                    {
                        match value {
                            $( $str => Ok($name::$variant), )*
                            _ => Err(E::invalid_value(::serde::de::Unexpected::Other(
                                &format!("unknown {} variant: {}", stringify!($name), value)
                            ), &self)),
                        }
                    }
                }

                // 从字符串反序列化枚举。
                deserializer.deserialize_str(Visitor)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                match value {
                    $( $name::$variant => $str.to_string(), )*
                }
            }
        }
    }
}

macro_rules! format_params {
    ($variant:ident : String) => {
        $variant
    };
    ($variant:ident : i64) => {
        format!("{}", $variant)
    };
    ($variant:ident : $type_id:ident) => {
        String::from($variant)
    };
}

macro_rules! query_params {
    ($name:ident { $($variant:ident($str:expr,$type_id:ident), )* }) => {
        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct $name {
            $(pub $variant: Option<$type_id>,)*
        }
        impl $name {
            pub(crate) fn queries(self) -> Vec<(String, String)> {
                let mut result = vec![];
                $(
                if let Some($variant) = self.$variant {
                    result.push(($str.to_owned(), format_params!($variant: $type_id)));
                }
                )*
                result
            }
            $(
            pub fn $variant(mut self, $variant: $type_id) -> Self {
                self.$variant = Some($variant);
                self
            }
            )*
        }
    };
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelfLinks {
    #[serde(rename = "self")]
    pub self_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelfAndRelatedLinks {
    #[serde(rename = "self")]
    pub self_field: String,
    pub related: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelatedLinks {
    pub related: String,
}

// Entity

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityResponse<T> {
    pub data: T,
    pub links: SelfLinks,
}

// Pages

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageResponse<T> {
    pub data: Vec<T>,
    pub links: PageLinks,
    pub meta: PageMeta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageLinks {
    #[serde(rename = "self")]
    pub self_field: String,
    pub next: Option<String>,
    pub first: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageMeta {
    pub paging: Paging,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Paging {
    pub total: i64,
    pub limit: i64,
}

//

enum_str!(ContentRightsDeclaration{
    DoesNotUseThirdPartyContent("DOES_NOT_USE_THIRD_PARTY_CONTENT"),
    UsesThirdPartyContent("USES_THIRD_PARTY_CONTENT"),
});

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct App {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: String,
    pub attributes: AppAttributes,
    pub relationships: AppRelationships,
    pub links: SelfLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppAttributes {
    pub name: String,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    pub sku: String,
    #[serde(rename = "primaryLocale")]
    pub primary_locale: String,
    #[serde(rename = "isOrEverWasMadeForKids")]
    pub is_or_ever_was_made_for_kids: bool,
    #[serde(rename = "subscriptionStatusUrl")]
    pub subscription_status_url: Option<String>,
    #[serde(rename = "subscriptionStatusUrlVersion")]
    pub subscription_status_url_version: Option<String>,
    #[serde(rename = "subscriptionStatusUrlForSandbox")]
    pub subscription_status_url_for_sandbox: Option<String>,
    #[serde(rename = "subscriptionStatusUrlVersionForSandbox")]
    pub subscription_status_url_version_for_sandbox: Option<String>,
    #[serde(rename = "availableInNewTerritories")]
    pub available_in_new_territories: bool,
    #[serde(rename = "contentRightsDeclaration")]
    pub content_rights_declaration: Option<ContentRightsDeclaration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppRelationships {
    #[serde(rename = "ciProduct")]
    pub ci_product: CiProduct,
    #[serde(rename = "betaTesters")]
    pub beta_testers: BetaTesters,
    #[serde(rename = "betaGroups")]
    pub beta_groups: BetaGroups,
    #[serde(rename = "appStoreVersions")]
    pub app_store_versions: AppStoreVersions,
    #[serde(rename = "preReleaseVersions")]
    pub pre_release_versions: PreReleaseVersions,
    #[serde(rename = "betaAppLocalizations")]
    pub beta_app_localizations: BetaAppLocalizations,
    pub builds: Builds,
    #[serde(rename = "betaLicenseAgreement")]
    pub beta_license_agreement: BetaLicenseAgreement,
    #[serde(rename = "betaAppReviewDetail")]
    pub beta_app_review_detail: BetaAppReviewDetail,
    #[serde(rename = "appInfos")]
    pub app_infos: AppInfos,
    #[serde(rename = "appClips")]
    pub app_clips: AppClips,
    #[serde(rename = "appPricePoints")]
    pub app_price_points: AppPricePoints,
    #[serde(rename = "pricePoints")]
    pub price_points: PricePoints,
    #[serde(rename = "endUserLicenseAgreement")]
    pub end_user_license_agreement: EndUserLicenseAgreement,
    #[serde(rename = "preOrder")]
    pub pre_order: PreOrder,
    pub prices: Prices,
    #[serde(rename = "appPriceSchedule")]
    pub app_price_schedule: AppPriceSchedule,
    #[serde(rename = "availableTerritories")]
    pub available_territories: AvailableTerritories,
    #[serde(rename = "appAvailability")]
    pub app_availability: AppAvailability,
    #[serde(rename = "inAppPurchases")]
    pub in_app_purchases: InAppPurchases,
    // Exists in apps, missing field `subscriptionGroups` in user_visible_apps
    #[serde(rename = "subscriptionGroups", default = "SubscriptionGroups::default")]
    pub subscription_groups: SubscriptionGroups,
    #[serde(rename = "gameCenterEnabledVersions")]
    pub game_center_enabled_versions: GameCenterEnabledVersions,
    #[serde(rename = "perfPowerMetrics")]
    pub perf_power_metrics: PerfPowerMetrics,
    #[serde(rename = "appCustomProductPages")]
    pub app_custom_product_pages: AppCustomProductPages,
    // Exists in apps, missing field `inAppPurchasesV2` in user_visible_apps
    #[serde(rename = "inAppPurchasesV2", default = "InAppPurchasesV2::default")]
    pub in_app_purchases_v2: InAppPurchasesV2,
    // Exists in apps, missing field `promotedPurchases` in user_visible_apps
    #[serde(rename = "promotedPurchases", default = "PromotedPurchases::default")]
    pub promoted_purchases: PromotedPurchases,
    #[serde(rename = "appEvents")]
    pub app_events: AppEvents,
    #[serde(rename = "reviewSubmissions")]
    pub review_submissions: ReviewSubmissions,
    #[serde(
        rename = "subscriptionGracePeriod",
        default = "SubscriptionGracePeriod::default"
    )]
    pub subscription_grace_period: SubscriptionGracePeriod,
    #[serde(rename = "customerReviews")]
    pub customer_reviews: CustomerReviews,
    // Exists in user_visible_apps, not exists in apps
    #[serde(
        rename = "appStoreVersionExperimentsV2",
        default = "AppStoreVersionExperimentsV2::default"
    )]
    pub app_store_version_experiments_v2: AppStoreVersionExperimentsV2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CiProduct {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BetaTesters {
    pub links: SelfLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BetaGroups {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppStoreVersions {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreReleaseVersions {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BetaAppLocalizations {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Builds {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BetaLicenseAgreement {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BetaAppReviewDetail {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppInfos {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppClips {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppPricePoints {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PricePoints {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndUserLicenseAgreement {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreOrder {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prices {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppPriceSchedule {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AvailableTerritories {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppAvailability {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InAppPurchases {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionGroups {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameCenterEnabledVersions {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerfPowerMetrics {
    pub links: RelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppCustomProductPages {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InAppPurchasesV2 {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromotedPurchases {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppEvents {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewSubmissions {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionGracePeriod {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerReviews {
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppStoreVersionExperimentsV2 {
    pub links: SelfAndRelatedLinks,
}

//

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
    // UNIVERSAL ?
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
    pub meta: PageMeta,
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BundleIdProfiles {
    pub meta: PageMeta,
    pub links: SelfAndRelatedLinks,
}

enum_str!(BundleIdsType{
    BundleIds("bundleIds"),
});

//

query_params!(CertificateQuery{
    fields_certificates("fields[certificates]",String),
    filter_id("filter[id]",String),
    filter_serial_number("filter[serialNumber]",String),
    limit("limit", i64),
    sort("sort",CertificateSort),
    filter_certificate_type("filter[certificateType]",CertificateType),
    filter_display_name("filter[displayName]",String),

});

enum_str!(CertificateSort {
    Id("id"),
    IdDesc("-id"),
    CertificateType("certificateType"),
    CertificateTypeDesc("-certificateType"),
    DisplayName("displayName"),
    DisplayNameDesc("-displayName"),
    SerialNumber("serialNumber"),
    SerialNumberDesc("-serialNumber"),
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Certificate {
    #[serde(rename = "type")]
    pub type_field: CertificatesType,
    pub id: String,
    pub attributes: CertificateAttributes,
    pub relationships: CertificateRelationships,
    pub links: SelfLinks,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateAttributes {
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[serde(rename = "certificateContent")]
    pub certificate_content: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub name: String,
    #[serde(rename = "csrContent")]
    pub csr_content: serde_json::Value,
    // null
    pub platform: Option<String>,
    // "IOS"/ null => IOS / MAC_OS ????
    #[serde(rename = "expirationDate")]
    pub expiration_date: DateTime<Utc>,
    #[serde(rename = "certificateType")]
    pub certificate_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateRelationships {
    #[serde(rename = "passTypeId")]
    pub pass_type_id: CertificateRelationshipsPassTypeId,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateRelationshipsPassTypeId {
    pub links: SelfAndRelatedLinks,
}

enum_str!(CertificatesType{
    Certificates("certificates"),
});

enum_str!(CertificateType{
    IosDevelopment("IOS_DEVELOPMENT"),
    IosDistribution("IOS_DISTRIBUTION"),
    MacAppDistribution("MAC_APP_DISTRIBUTION"),
    MacInstallerDistribution("MAC_INSTALLER_DISTRIBUTION"),
    MacAppDevelopment("MAC_APP_DEVELOPMENT"),
    DeveloperIdKext("DEVELOPER_ID_KEXT"),
    DeveloperIdApplication("DEVELOPER_ID_APPLICATION"),
    Development("DEVELOPMENT"),
    Distribution("DISTRIBUTION"),
    PassTypeId("PASS_TYPE_ID"),
    PassTypeIdWithNfc("PASS_TYPE_ID_WITH_NFC"),
});

// Profile

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
    pub meta: PageMeta,
    pub links: SelfAndRelatedLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Devices {
    pub meta: PageMeta,
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

// profile create

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
    pub type_field: BundleIdsType,
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

// devices

query_params!(DeviceQuery{
    fields_devices("fields[devices]",String),
    filter_id("filter[id]",String),
    filter_name("filter[name]",String),
    filter_platform("filter[platform]",BundleIdPlatform),
    filter_status("filter[status]",DeviceStatus),
    filter_udid("filter[udid]",String),
    limit("limit", i64),
    sort("sort", DeviceSort),
});

enum_str!(DeviceSort{
    Id("id"),
    IdDesc("-id"),
    Name("name"),
    NameDesc("-name"),
    Platform("platform"),
    PlatformDesc("-platform"),
    Status("status"),
    StatusDesc("-status"),
    Udid("udid"),
    UdidDesc("-udid"),
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: String,
    pub attributes: DeviceAttributes,
    pub links: SelfLinks,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceAttributes {
    #[serde(rename = "addedDate")]
    pub added_date: DateTime<Utc>,
    // "2022-12-10T12:02:45.000+00:00"
    pub name: String,
    #[serde(rename = "deviceClass")]
    pub device_class: DeviceClass,
    pub model: Option<String>,
    pub udid: String,
    pub platform: BundleIdPlatform,
    pub status: DeviceStatus,
}

// Device Create

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceCreateRequest {
    pub data: DeviceCreateRequestData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceCreateRequestData {
    #[serde(rename = "type")]
    pub type_field: DeviceType,
    pub attributes: DeviceCreateRequestDataAttributes,
}

enum_str!(DeviceType {
    Devices("devices"),
});

enum_str!(DeviceStatus {
    Enabled("ENABLED"),
    Disabled("DISABLED"),
    Processing("PROCESSING"),
    Ineligible("INELIGIBLE"),
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceCreateRequestDataAttributes {
    pub name: String,
    pub platform: BundleIdPlatform,
    pub udid: String,
}

enum_str!(DeviceClass {
    AppleWatch("APPLE_WATCH"),
    Ipad("IPAD"),
    Iphone("IPHONE"),
    Ipod("IPOD"),
    AppleTv("APPLE_TV"),
    Mac("MAC"),
});

enum_str!(BundleIdPlatform {
    Ios("IOS"),
    MacOS("MAC_OS"),
});

//

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

enum_str!(AppsType{
    Apps("Apps"),
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateCreateRequest {
    pub data: CertificateCreateRequestData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateCreateRequestData {
    #[serde(rename = "type")]
    pub type_field: CertificatesType,
    pub attributes: CertificateCreateRequestDataAttributes,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateCreateRequestDataAttributes {
    #[serde(rename = "certificateType")]
    pub certificate_type: CertificateType,
    #[serde(rename = "csrContent")]
    pub csr_content: String,
}

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
