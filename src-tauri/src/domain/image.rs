pub struct Image {
    id: String,
    name: String,
}

impl Image {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
