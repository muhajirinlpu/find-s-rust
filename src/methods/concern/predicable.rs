pub struct Prediction<T> {
    pub value: T,
    pub confidence: f64,
}

pub trait Predicable<T> {
    /// predict is a method for processing test data input by user
    /// return some possibility of a test result between 0 and 1
    fn predict(&self) -> Vec<Prediction<T>>;
}
