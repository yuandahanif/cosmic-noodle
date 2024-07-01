pub struct Config {
    name: String,
    version: String,
    author: String,
    qualifier: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            name: String::from("app"),
            version: String::from("0.1.0"),
            author: String::from("author"),
            qualifier: String::from("com"),
        }
    }
}

impl Config {
    pub fn new(name: String, version: String, author: String, qualifier: String) -> Self {
        Config {
            name,
            version,
            author,
            qualifier,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn qualifier(&self) -> &str {
        &self.qualifier
    }
}
