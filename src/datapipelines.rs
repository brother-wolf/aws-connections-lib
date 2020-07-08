use ::rusoto_datapipeline::DataPipelineClient;
use ::rusoto_core::{Region, CredentialsError, HttpClient};
use ::rusoto_credential::{ChainProvider, ProfileProvider};
use std::str::FromStr;

pub fn get_datapipeline_client(aws_profile: &str, region: &str) -> Option<DataPipelineClient> {

    match Region::from_str(region) {
        Ok(aws_region) => {
            if aws_profile.is_empty() {
                ChainProvider::new();
                Some(DataPipelineClient::new(aws_region))
            } else {
                let _profile_provider: Result<ProfileProvider,CredentialsError> = ProfileProvider::new();
                match _profile_provider {
                    Ok(mut _prov) => {
                        _prov.set_profile(aws_profile);
                        Some(DataPipelineClient::new_with(HttpClient::new().expect("failed to create request dispatcher"), _prov, aws_region))
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

