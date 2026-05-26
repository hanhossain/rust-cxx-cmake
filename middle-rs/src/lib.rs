pub struct SomethingOpaque {
    owner: String,
}

impl SomethingOpaque {
    pub fn new() -> Self {
        Self {
            owner: "rust".into(),
        }
    }

    pub fn owner(&self) -> String {
        println!(
            "[middle-rs::SomethingOpaque::owner] owner is '{}'",
            self.owner
        );
        self.owner.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something_opaque_owner() {
        let s = SomethingOpaque::new();
        let owner = s.owner();
        assert_eq!(owner, "rust");
    }
}
