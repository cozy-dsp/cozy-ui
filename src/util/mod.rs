use once_cell::sync::Lazy;

// ideally, i'd like this to be done at compile time, but i don't feel like writing a build script rn
pub static CIRCLE_POINTS: Lazy<[(f32, f32); 360]> = Lazy::new(|| {
    let mut result = [(0.0, 0.0); 360];
    #[allow(clippy::cast_precision_loss)]
    for (idx, sin_cos) in result.iter_mut().enumerate() {
        // if a 32 bit float cant represent whole numbers up to 360 degress, i'll eat my tail (shoes for non furries)
        *sin_cos = (idx as f32).to_radians().sin_cos();
    }
    result
});
