pub mod button;
pub mod knob;

fn get<T, GetSet>(getter: &mut GetSet) -> T
where
    GetSet: FnMut(Option<T>) -> T,
{
    getter(None)
}

fn set<T, GetSet>(setter: &mut GetSet, value: T)
where
    GetSet: FnMut(Option<T>) -> T,
{
    setter(Some(value));
}
