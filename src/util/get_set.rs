pub enum Operation<T> {
    Get,
    Set(T),
}

impl<T> From<Option<T>> for Operation<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => Self::Set(v),
            None => Self::Get,
        }
    }
}
