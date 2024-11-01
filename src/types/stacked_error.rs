#[derive(Debug)]
pub struct StackedError {
    stack: Vec<String>,
}

impl StackedError {
    pub fn new(what: String) -> Self {
        Self { stack: vec![what] }
    }

    pub fn chain(mut self, e: String) -> Self {
        self.stack.push(e);
        self
    }

    pub fn join(&self, sep: &str) -> String {
        self.stack.join(sep)
    }
}

#[macro_export]
macro_rules! format_stacked_err {
    ($($args:tt)*) => {
        Err(StackedError::new(format!($($args)*)))
    };
}

#[macro_export]
macro_rules! stack_error {
    ($($args:tt)*) => {
        |err: StackedError| err.chain(format!($($args)*))
    };
}
