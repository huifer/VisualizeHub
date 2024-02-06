#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::time::Duration;

    use futures::prelude::*;
    use influxdb2::Client;

    use db_show::config::influxdb_config::InfluxDBUserPassword;
    use db_show::op::influxdb2_op::Influxdb2Operation;

    #[tokio::test]
    async fn example() -> Result<(), Box<dyn std::error::Error>> {
        let org = "myorg";
        let token = "eh5fkkCC-diSWJRe4a-Oi3yJ8Lcnb1d_iciDvGvHWeF8beoSALcE8S9fJgIqAXYEfORPRmLD6vLHn6Xwo_RySQ==";
        let influx_url = "http://localhost:8086";


        let client = influxdb2::Client::new(influx_url, org, "mytoken");
        let operation = Influxdb2Operation::new(InfluxDBUserPassword {
            url: influx_url.to_string(),
            name: "asf".to_string(),
            auth_token: "mytoken".to_string(),
            org: "myorg".to_string(),
            version: "2".to_string(),
        });


        let result = operation.get_list_buckets(10, 0).await;
        // dbg!(result);
        let result1 = operation.get_list_organizations(10, 0).await;
        // dbg!(result1);


        let result2 = operation.get_list_measurements("test".to_string()).await;
        dbg!(result2);

        let result3 = operation.get_list_fields("test".to_string(), "cpu_load_short".to_string()).await;
        dbg!(result3);
        let result4 = operation.get_list_measurement_tag_keys("test".to_string(), "cpu_load_short".to_string()).await;
        dbg!(result4);
        // write_sample_data(bucket, client).await?;
        Ok(())
    }


    async fn write_sample_data(bucket: &str, client: Client) -> Result<(), Box<dyn Error>> {
        for index in 0..100 {
            let points = vec![
                influxdb2::models::DataPoint::builder("cpu_load_short")
                    .tag("host", "server01")
                    .tag("region", "us-west")
                    .field("value", 0.64)
                    .build()?,
                influxdb2::models::DataPoint::builder("cpu_load_short")
                    .tag("host", "server01")
                    .field("value", 27.99)
                    .build()?,
            ];
            client.write(bucket, stream::iter(points)).await?;
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("当前处理索引: {}", index);
        }
        Ok(())
    }
}