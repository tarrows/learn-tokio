use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
  pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3(x, y, z)
  }

  // for coordinate
  pub fn x(self) -> f64 {
    self.0
  }
  pub fn y(self) -> f64 {
    self.1
  }
  pub fn z(self) -> f64 {
    self.2
  }

  // for color
  pub fn r(self) -> f64 {
    self.0
  }
  pub fn g(self) -> f64 {
    self.1
  }
  pub fn b(self) -> f64 {
    self.2
  }
}

impl ops::Add for Vec3 {
  type Output = Vec3;

  fn add(self, _rhs: Vec3) -> Vec3 {
    Vec3(
      self.x() + _rhs.x(),
      self.y() + _rhs.y(),
      self.z() + _rhs.z(),
    )
  }
}

impl ops::Sub for Vec3 {
  type Output = Vec3;

  fn sub(self, _rhs: Vec3) -> Vec3 {
    Vec3(
      self.x() - _rhs.x(),
      self.y() - _rhs.y(),
      self.z() - _rhs.z(),
    )
  }
}
