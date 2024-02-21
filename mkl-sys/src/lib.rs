#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Bindings
pub const BINDINGS: &str = include_str!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod test {
    use rand::distributions::{Distribution, Uniform};

    #[test]
    pub fn test_cos() {
        // f32
        for &n in &[100, 1000, 10000] {
            let in_ = {
                let mut rng = rand::thread_rng();
                let between = Uniform::try_from(0.0..2.0 * std::f32::consts::PI).unwrap();
                let mut buf = vec![0.0; n];
                for val in buf.iter_mut() {
                    *val = between.sample(&mut rng);
                }
                buf
            };

            let mut out = vec![0.0_f32; n];

            (0..n).for_each(|n| unsafe {
                crate::vsCos(n as i32, in_.as_ptr(), out.as_mut_ptr());
            });
        }
    }
}