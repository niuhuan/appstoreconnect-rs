use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

use super::{BundleIdPlatform, SelfLinks};

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
    pub name: String,
    #[serde(rename = "deviceClass")]
    pub device_class: DeviceClass,
    pub model: Option<String>,
    pub udid: String,
    pub platform: BundleIdPlatform,
    pub status: DeviceStatus,
}

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

