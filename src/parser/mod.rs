use std::collections::HashMap;

// Define a trait for configuration parsing.
pub trait ConfigParser {
    fn parse(&self, source: &str) -> Result<HashMap<String, String>, &'static str>;
}

// Default implementation for file-based configuration.
pub struct FileConfigParser {
    delimiter: char,
}

impl FileConfigParser {
    pub fn new(delimiter: char) -> Self {
        Self { delimiter }
    }
}

impl ConfigParser for FileConfigParser {
    fn parse(&self, source: &str) -> Result<HashMap<String, String>, &'static str> {
        let content = std::fs::read_to_string(source).map_err(|_| "Error reading file")?;
        let mut config_map = HashMap::new();

        for line in content.lines() {
            let mut parts = line.splitn(2, self.delimiter);
            if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
                config_map.insert(k.trim().to_owned(), v.trim().to_owned());
            }
        }

        Ok(config_map)
    }
}

pub struct Config {
    data: HashMap<String, String>,
}

impl Config {
    pub fn new<P: ConfigParser>(parser: P, source: &str) -> Result<Self, &'static str> {
        let data = parser.parse(source)?;
        Ok(Self { data })
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }
}
