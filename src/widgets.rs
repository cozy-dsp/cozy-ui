pub mod button;
pub mod knob;
pub mod slider;

pub use button::toggle;
pub use knob::Knob;
pub use slider::slider;

use crate::util::get_set::Operation;

fn get<T, GetSet>(operator: &mut GetSet) -> T
where
    GetSet: FnMut(Operation<T>) -> T,
{
    operator(Operation::Get)
}

fn set<T, GetSet>(operator: &mut GetSet, value: T)
where
    GetSet: FnMut(Operation<T>) -> T,
{
    operator(Operation::Set(value));
}
