use std::sync::{Arc, Mutex};

pub struct Guard<T> {
    inner: Arc<Mutex<T>>,
}

impl<T> Guard<T> {
    pub fn new(t: T) -> Self {
        Guard{inner: Arc::new(Mutex::new(t))}
    }

    pub fn execute<F: FnMut(Arc<Mutex<T>>)>(&self, mut callback: F) {
        callback(self.inner.clone());
    }

    pub fn clone(&self) -> Self {
        Guard{inner: self.inner.clone()}
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
