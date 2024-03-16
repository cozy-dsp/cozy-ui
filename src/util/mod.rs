use once_cell::sync::Lazy;

// ideally, i'd like this to be done at compile time, but i don't feel like writing a build script rn
pub(crate) static CIRCLE_POINTS: Lazy<[(f32, f32); 360]> = Lazy::new(|| {
    let mut result = [(0.0, 0.0); 360];
    for (idx, sin_cos) in result.iter_mut().enumerate() {
        *sin_cos = (idx as f32).to_radians().sin_cos();
    }
    result
});