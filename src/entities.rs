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
pub struct MetaLinks {
    #[serde(rename = "self")]
    pub self_field: String,
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
    pub platform: String, // UNIVERSAL ?
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
    pub links: MetaLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BundleIdProfiles {
    pub meta: PageMeta,
    pub links: MetaLinks,
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
    pub csr_content: serde_json::Value, // null
    pub platform: Option<String>, // "IOS"/ null => IOS / MAC_OS ????
    #[serde(rename = "expirationDate")]
    pub expiration_date: String,
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
    pub links: MetaLinks,
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
    pub links: MetaLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Certificates {
    pub meta: PageMeta,
    pub links: MetaLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Devices {
    pub meta: PageMeta,
    pub links: MetaLinks,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: String,
    pub attributes: DeviceAttributes,
    pub links: SelfLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceAttributes {
    #[serde(rename = "addedDate")]
    pub added_date: DateTime<Utc>, // "2022-12-10T12:02:45.000+00:00"
    pub name: String,
    #[serde(rename = "deviceClass")]
    pub device_class: String,
    pub model: Option<String>,
    pub udid: String,
    pub platform: String,
    pub status: String,
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

enum_str!(BundleIdPlatform {
    Ios("IOS"),
    MacOS("MAC_OS"),
});

//
