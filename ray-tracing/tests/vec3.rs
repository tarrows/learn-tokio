// use ray_tracing::vec3::Vec3;
// use std::f64;

// to run test in this file: `$ cargo test --test vec3`
#[cfg(test)]
mod test {
  #[test]
  fn overload_operators() {
    let a = ray_tracing::vec3::Vec3::new(0.1, 0.2, 0.3);
    let b = ray_tracing::vec3::Vec3::new(0.0, 0.4, 0.3);
    let c = ray_tracing::vec3::Vec3::new(0.1, 0.6, 0.6);
    assert!((c.x() - (a + b).x()) < std::f64::EPSILON);
  }
}
