use influxdb2::api::buckets::ListBucketsRequest;
use influxdb2::api::organization::ListOrganizationRequest;
use influxdb2::models::{Buckets, Organizations};
use influxdb2::{Client, RequestError};

use crate::config::influxdb_config::InfluxDBUserPassword;

pub struct Influxdb2Operation {
    pub client: Client,
}

impl Influxdb2Operation {
    pub fn new(param: InfluxDBUserPassword) -> Self {
        let client = influxdb2::Client::new(param.url, param.org, param.auth_token);
        Self { client }
    }

    pub async fn get_list_buckets(&self, limit: u8, offset: u64) -> Result<Buckets, RequestError> {
        let x = self
            .client
            .list_buckets(Some(ListBucketsRequest {
                after: None,
                id: None,
                limit: Some(limit),
                name: None,
                offset: Some(offset),
                org: None,
                org_id: None,
            }))
            .await;
        x
    }
    pub async fn get_list_organizations(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Organizations, RequestError> {
        let x = self
            .client
            .list_organizations(ListOrganizationRequest {
                descending: None,
                limit: Some(limit),
                offset: Some(offset),
                org: None,
                org_id: None,
                user_id: None,
            })
            .await;
        x
    }

    /// 先用 get_list_measurements 在用 get_list_fields
    pub async fn get_list_measurements(&self, bucket: String) -> Result<Vec<String>, RequestError> {
        let result = self
            .client
            .list_measurements(bucket.as_str(), None, None)
            .await;
        result
    }
    pub async fn get_list_fields(
        &self,
        bucket: String,
        measurement: String,
    ) -> Result<Vec<String>, RequestError> {
        let result = self
            .client
            .list_measurement_field_keys(bucket.as_str(), measurement.as_str(), None, None)
            .await;
        result
    }
    pub async fn get_list_measurement_tag_keys(
        &self,
        bucket: String,
        measurement: String,
    ) -> Result<Vec<String>, RequestError> {
        let result = self
            .client
            .list_measurement_tag_keys(bucket.as_str(), measurement.as_str(), None, None)
            .await;
        result
    }
}
