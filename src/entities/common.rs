use serde_derive::{Deserialize, Serialize};

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
    pub links: PagedDocumentLinks,
    pub meta: PagingInformation,
}

impl<T> PageResponse<T>
where
    T: for<'de> serde::Deserialize<'de>,
{
    pub fn pager(self, client: &'_ crate::client::Client) -> crate::pager::Pager<'_, T> {
        crate::pager::Pager::new(client, self)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PagedDocumentLinks {
    #[serde(rename = "self")]
    pub self_field: String,
    pub next: Option<String>,
    pub first: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PagingInformation {
    pub paging: Paging,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Paging {
    pub total: i64,
    pub limit: i64,
}

enum_str!(BundleIdPlatform {
    Ios("IOS"),
    MacOS("MAC_OS"),
});

