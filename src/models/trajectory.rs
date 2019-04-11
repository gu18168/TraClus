//! 轨迹，从文件中直接读取生成的对象
use crate::{
  models::{
    point::Point
  }
};

pub struct Trajectory {
  id: usize,
  points: Vec<Point>
}

impl Trajectory {
  /// 初始化一条轨迹
  pub fn new(id: usize) -> Self {
    Self {
      id,
      points: Vec::new()
    }
  }

  /// 获得轨迹的 id
  pub fn get_id(&self) -> usize {
    self.id
  }

  /// 给轨迹添加轨迹点
  pub fn add_point(&mut self, point: Point) {
    self.points.push(point);
  }

  /// 获得轨迹所有轨迹点的所有权
  pub fn get_points(self) -> Vec<Point> {
    self.points
  }

  /// 获取指定索引的轨迹点
  pub fn get_point(&self, index: usize) -> Option<&Point> {
    self.points.get(index)
  }

  /// 获取轨迹点集的长度
  pub fn get_points_len(&self) -> usize {
    self.points.len()
  }
}