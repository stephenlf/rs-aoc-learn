use std::sync::Arc;

impl super::Forest {
    fn hey(&self) {
        let a = Arc::new(self);
    }
}