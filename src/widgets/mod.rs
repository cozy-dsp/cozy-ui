pub mod button;
pub mod knob;
pub mod slider;

pub use button::toggle;
pub use knob::Knob;
pub use slider::slider;

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
