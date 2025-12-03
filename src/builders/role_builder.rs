pub struct RoleBuilder {}

impl RoleBuilder {
    pub fn new() -> Self {
        RoleBuilder {}
    }

    pub async fn build(&self) {
        // Implementation for building a role goes here
    }
}

impl Default for RoleBuilder {
    fn default() -> Self {
        Self::new()
    }
}
