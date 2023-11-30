use base64::DecodeError;

use crate::client::{Client, ClientBuilder};
use crate::entities::{
    BundleIdPlatform, BundleIdQuery, BundleIdsType, CertificateCreateRequest,
    CertificateCreateRequestData, CertificateCreateRequestDataAttributes, CertificateQuery,
    CertificateType, CertificatesType, DeviceCreateRequest, DeviceCreateRequestData,
    DeviceCreateRequestDataAttributes, DeviceQuery, DeviceType, ProfileCreateRequest,
    ProfileCreateRequestAttributes, ProfileCreateRequestData,
    ProfileCreateRequestDataRelationshipsBundleId,
    ProfileCreateRequestDataRelationshipsBundleIdData,
    ProfileCreateRequestDataRelationshipsCertificates,
    ProfileCreateRequestDataRelationshipsCertificatesData,
    ProfileCreateRequestDataRelationshipsDevices, ProfileCreateRequestDataRelationshipsDevicesData,
    ProfileCreateRequestRelationships, ProfileCreateRequestType, ProfileQuery, ProfileType,
    UserVisibleAppsQuery, UsersQuery,
};
use crate::error::{Error, Result};

impl From<DecodeError> for Error {
    fn from(value: DecodeError) -> Self {
        Self::Other(Box::new(value))
    }
}

fn gen_client() -> Result<Client> {
    ClientBuilder::default()
        .with_iss(env!("iss"))
        .with_kid(env!("kid"))
        .with_ec_der(base64::decode(env!("ec_der"))?)
        .build()
}

fn print<T>(result: Result<T>)
where
    T: serde::Serialize + Send + Sync,
{
    match result {
        Ok(t) => match serde_json::to_string(&t) {
            Ok(text) => println!("{}", text),
            Err(err) => panic!("{}", err),
        },
        Err(err) => {
            panic!("{}", err);
        }
    }
}

#[tokio::test]
async fn test_apps() -> Result<()> {
    print(gen_client()?.apps(BundleIdQuery::default()).await);
    Ok(())
}

#[tokio::test]
async fn test_bundle_ids() -> Result<()> {
    print(gen_client()?.bundle_ids(BundleIdQuery::default()).await);
    Ok(())
}

#[tokio::test]
async fn test_certificates() -> Result<()> {
    print(
        gen_client()?
            .certificates(CertificateQuery::default())
            .await,
    );
    Ok(())
}

#[tokio::test]
async fn test_create_profile() -> Result<()> {
    print(
        gen_client()?
            .create_profile(ProfileCreateRequest {
                data: ProfileCreateRequestData {
                    attributes: ProfileCreateRequestAttributes {
                        name: "profileName".to_string(),
                        profile_type: ProfileType::IosAppAdhoc,
                    },
                    relationships: ProfileCreateRequestRelationships {
                        bundle_id: ProfileCreateRequestDataRelationshipsBundleId {
                            data: ProfileCreateRequestDataRelationshipsBundleIdData {
                                id: "FJXB650000".to_string(),
                                type_field: BundleIdsType::BundleIds,
                            },
                        },
                        certificates: ProfileCreateRequestDataRelationshipsCertificates {
                            data: vec![ProfileCreateRequestDataRelationshipsCertificatesData {
                                id: "87792Q0000".to_string(),
                                type_field: CertificatesType::Certificates,
                            }],
                        },
                        devices: Some(ProfileCreateRequestDataRelationshipsDevices {
                            data: vec![ProfileCreateRequestDataRelationshipsDevicesData {
                                id: "25D9760000".to_string(),
                                type_field: DeviceType::Devices,
                            }],
                        }),
                    },
                    type_field: ProfileCreateRequestType::Profiles,
                },
            })
            .await,
    );
    Ok(())
}

#[tokio::test]
async fn test_profiles() -> Result<()> {
    print(gen_client()?.profiles(ProfileQuery::default()).await);
    Ok(())
}

#[tokio::test]
async fn test_delete_profile() -> Result<()> {
    print(gen_client()?.delete_profile("4H6J3W0000").await);
    Ok(())
}

#[tokio::test]
async fn test_devices() -> Result<()> {
    print(
        gen_client()?
            .devices(DeviceQuery {
                filter_name: Some("mini".to_string()),
                ..Default::default()
            })
            .await,
    );
    Ok(())
}

#[tokio::test]
async fn test_register_devices() -> Result<()> {
    print(
        gen_client()?
            .register_new_device(DeviceCreateRequest {
                data: DeviceCreateRequestData {
                    type_field: DeviceType::Devices,
                    attributes: DeviceCreateRequestDataAttributes {
                        name: "LiLi".to_string(),
                        platform: BundleIdPlatform::Ios,
                        udid: "00008020-000000000000002E".to_string(),
                    },
                },
            })
            .await,
    );
    Ok(())
}

#[tokio::test]
async fn test_revokec_certificate() -> Result<()> {
    print(
        gen_client()?
            .revoke_certificate("87792Q0000".to_string())
            .await,
    );
    Ok(())
}

#[tokio::test]
async fn test_users() -> Result<()> {
    print(gen_client()?.users(UsersQuery::default()).await);
    Ok(())
}

#[tokio::test]
async fn test_user_information() -> Result<()> {
    print(
        gen_client()?
            .user_information("xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")
            .await,
    );
    Ok(())
}

#[tokio::test]
async fn test_user_visible_apps() -> Result<()> {
    print(
        gen_client()?
            .user_visible_apps(
                "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
                UserVisibleAppsQuery::default(),
            )
            .await,
    );
    Ok(())
}

#[tokio::test]
async fn test_create_certificate() -> Result<()> {
    let c = std::fs::read_to_string("target/tmp/csr.csr").unwrap();
    print(
        gen_client()?
            .create_certificate(CertificateCreateRequest {
                data: CertificateCreateRequestData {
                    type_field: CertificatesType::Certificates,
                    attributes: CertificateCreateRequestDataAttributes {
                        certificate_type: CertificateType::MacAppDevelopment,
                        csr_content: c,
                    },
                },
            })
            .await,
    );
    Ok(())
}
