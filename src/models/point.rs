//! 二维点，包含 (x, y) 坐标值
pub struct Point {
  x: f64,
  y: f64,
  sog: f64,
  timestamp: i64
}

impl Point {
  /// 创建一个初始二维点
  pub fn init() -> Self {
    Self {
      x: 0.0,
      y: 0.0,
      sog: 0.0,
      timestamp: 0
    }
  }

  /// 创建一个二维点
  pub fn new(x: f64, y: f64, sog: f64, timestamp: i64) -> Self {
    Self {
      x,
      y,
      sog, 
      timestamp
    }
  }

  pub fn get_sog(&self) -> f64 {
    self.sog
  }

  pub fn get_timestamp(&self) -> i64 {
    self.timestamp
  }

  /// 获得点的 x 坐标
  pub fn get_x(&self) -> f64 {
    self.x
  }

  /// 获得点的 y 坐标
  pub fn get_y(&self) -> f64 {
    self.y
  }

  /// 设置点的 x 坐标
  pub fn set_x(&mut self, x: f64) {
    self.x = x;
  }

  /// 设置点的 y 坐标
  pub fn set_y(&mut self, y: f64) {
    self.y = y;
  }

  pub fn set_sog(&mut self, sog: f64) {
    self.sog = sog;
  }
}