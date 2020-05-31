use game_lib::HwError;

pub struct Config {
    config_json: serde_json::Value,
}

impl Config {
    pub fn new() -> Result<Self, HwError> {
        let config_path = Self::get_config_file_path()?;
        Self::from_file(&config_path)
    }

    pub fn from_file(config_path: &str) -> Result<Self, HwError> {
        let config_json = Self::read_json_config(&config_path)?;
        Ok(Self { config_json })
    }
    pub fn get_server_address(&self) -> Result<String, HwError> {
        let address = self.config_json.get("address").ok_or(HwError::ConfigError(
            "coreProps.json - file should have address key".to_string(),
        ))?;
        if let serde_json::Value::String(address) = address {
            Ok(address.to_string())
        } else {
            Err(HwError::ConfigError(
                "coreProps.json - invalid file format - missing address".to_string(),
            ))
        }
    }

    fn get_config_file_path() -> Result<String, HwError> {
        let program_data_path = std::env::var("PROGRAMDATA")
            .map_err(|_| HwError::ConfigError("env PROGRAMDATA not set".to_string()))?;
        Ok(program_data_path + "/SteelSeries/SteelSeries Engine 3/coreProps.json")
    }

    fn read_json_config(file: &str) -> Result<serde_json::Value, HwError> {
        let file = std::fs::File::open(file)
            .map_err(|_| HwError::ConfigError("Cannot open coreProps.json!".to_string()))?;
        serde_json::from_reader(file).map_err(|_| {
            HwError::ConfigError("coreProps.json - file should be proper JSON".to_string())
        })
    }
}
