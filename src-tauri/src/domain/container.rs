pub struct Container {
    name: String,
}

impl Container {
    pub fn name(&self) -> &str {
        &self.name
    }
}
