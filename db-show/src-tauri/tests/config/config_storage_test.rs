#[cfg(test)]
mod tests {
    use db_show::config::mysql_config::MysqlUserPassword;
    use db_show::config::mysql_config_storage::{MySQLStorageManager, MysqlStorageService};

    #[test]
    fn test() {
        let mut manager = MySQLStorageManager::new();
        manager.write_config_json();
        manager.add_username_password_model(MysqlUserPassword {
            username: "1".to_string(),
            password: "".to_string(),
            host: "".to_string(),
            port: 1,
            name: "测试数据库".to_string(),
        });
        println!("aa")
    }
}
