pub enum Operation<T> {
    Get,
    Set(T),
}

impl<T> From<Option<T>> for Operation<T> {
    fn from(value: Option<T>) -> Self {
        value.map_or_else(|| Self::Get, |v| Self::Set(v))
    }
}
