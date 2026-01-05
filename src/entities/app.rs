use serde_derive::{Deserialize, Serialize};

use super::{RelatedLinks, SelfAndRelatedLinks, SelfLinks};

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
    #[serde(
        rename = "promotedPurchases",
        default = "PromotedPurchases::default"
    )]
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

enum_str!(AppsType{
    Apps("Apps"),
});

