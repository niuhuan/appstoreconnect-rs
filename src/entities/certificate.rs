use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

use super::{SelfAndRelatedLinks, SelfLinks};

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
    pub platform: Option<String>,
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

