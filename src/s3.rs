use std::str::FromStr;

use rusoto_core::{HttpClient, Region};
use rusoto_credential::{ChainProvider, CredentialsError, ProfileProvider};
use rusoto_s3::S3Client;

pub fn get_client(aws_profile: &str, region: &str) -> Option<S3Client> {

    match Region::from_str(region) {
        Ok(aws_region) => {
            if aws_profile.is_empty() {
                ChainProvider::new();
                Some(S3Client::new(aws_region))
            } else {
                let _profile_provider: Result<ProfileProvider,CredentialsError> = ProfileProvider::new();
                match _profile_provider {
                    Ok(mut _prov) => {
                        _prov.set_profile(aws_profile);
                        Some(S3Client::new_with(HttpClient::new().expect("failed to create request dispatcher"), _prov, aws_region))
                    },
                    Err(_e) => {
                        println!("error in getting profile provider: {:?}", _e);
                        None
                    },
                }
            }
        },
        Err(_e) => None,
    }   
}
