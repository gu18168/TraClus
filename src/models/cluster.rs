use crate::{
  models::{
    point::Point
  }
};

pub struct Cluster {
  id: usize,
  points: Vec<Point>
}

impl Cluster {
  pub fn new(id: usize, points: Vec<Point>) -> Self {
    Self {
      id,
      points
    }
  }

  /// 获得簇的 id
  pub fn get_id(&self) -> usize {
    self.id
  }

  /// 获得簇的长度
  pub fn get_len(&self) -> usize {
    self.points.len()
  }

  /// 获得簇内所有点
  pub fn get_points(&self) -> &Vec<Point> {
    &self.points
  }

  /// 簇内增加点
  pub fn add_point(&mut self, point: Point) {
    self.points.push(point);
  }
}