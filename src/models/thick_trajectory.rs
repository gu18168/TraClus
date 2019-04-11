//! 划分后轨迹，经过 MDL 算法浓缩过的轨迹
use crate::{
  models::{
    point::Point
  }
};

pub struct ThickTrajectory {
  id: usize,
  partition_points: Vec<Point>
}

impl ThickTrajectory {
  /// 创建一条划分后轨迹
  pub fn new(id: usize, partition_points: Vec<Point>) -> Self {
    Self {
      id,
      partition_points
    }
  }

  /// 获得轨迹长度
  pub fn get_len(&self) -> usize {
    self.partition_points.len()
  }

  /// 获得轨迹的 id
  pub fn get_id(&self) -> usize {
    self.id
  }

  /// 获得指定索引的划分点
  pub fn get_partition_point(&self, index: usize) -> Option<&Point> {
    self.partition_points.get(index)
  }
}