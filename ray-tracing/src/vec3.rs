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

  pub fn len(self) -> f64 {
    (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
  }

  pub fn squared_len(self) -> f64 {
    self.0 * self.0 + self.1 * self.1 + self.2 * self.2
  }

  pub fn unit(self) -> Vec3 {
    if self.0 == 0.0 && self.1 == 0.0 && self.2 == 0.0 {
      Vec3(0.0, 0.0, 0.0)
    } else {
      let k = 1.0 / self.len();
      Vec3(self.0 * k, self.1 * k, self.2 * k)
    }
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

impl ops::Mul for Vec3 {
  type Output = Vec3;

  fn mul(self, _rhs: Vec3) -> Vec3 {
    Vec3(
      self.x() * _rhs.x(),
      self.y() * _rhs.y(),
      self.z() * _rhs.z(),
    )
  }
}

impl ops::Div for Vec3 {
  type Output = Vec3;

  fn div(self, _rhs: Vec3) -> Vec3 {
    Vec3(
      self.x() / _rhs.x(),
      self.y() / _rhs.y(),
      self.z() / _rhs.z(),
    )
  }
}
