pub struct Container {
    name: String,
    id: String,
    status: String,
    version: String,
    country: String,
}

impl Container {
    pub fn new(name: String, id: String, status: String, version: String, country: String) -> Self {
        Self {
            name,
            id,
            status,
            version,
            country,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn country(&self) -> &str {
        &self.country
    }
}
