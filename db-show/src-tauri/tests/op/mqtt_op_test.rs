// #[cfg(test)]
// mod tests {
//     use std::collections::HashMap;
//     use std::sync::Arc;
//     use std::thread;
//     use rumqtt::{MqttClient, MqttOptions, QoS, ReconnectOptions, SecurityOptions};
//     use std::time::Duration;
//     #[tokio::test]
//      async fn  test_server_info() {
//
//         let username = "admin";
//
//         let password = "lxs@2023";
//
//         let security_options = SecurityOptions::UsernamePassword(username.to_string(), password.to_string());
//
//         let mqtt_options = MqttOptions::new("test-pubsub1", "1", 1883)
//             .set_security_opts(security_options)
//             ;
//         let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
//
//         mqtt_client.subscribe("hello/world", QoS::AtLeastOnce).unwrap();
//         let sleep_time = Duration::from_secs(1);
//         thread::spawn(move || {
//             for i in 0..100 {
//                 let payload = format!("publish {}", i);
//                 thread::sleep(sleep_time);
//                 mqtt_client.publish("hello/world", QoS::AtLeastOnce, false, payload).unwrap();
//             }
//         });
//
//         let mut callbacks: HashMap<String, fn(Arc<Vec<u8>>) -> ()> = HashMap::new();
//         callbacks.insert("hello/world".to_string(), foo);
//
//
//         // 等待一些时间，以便观察输出
//         let x = tokio::time::sleep(Duration::from_secs(300)).await;
//
//     }
//
//     fn foo(payload: Arc<Vec<u8>>)
//     {
//         let string_data: String = String::from_utf8_lossy(payload.as_ref()).to_string();
//
//         println!("Got foo message: {:?}", string_data);
//     }
// }
