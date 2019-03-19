use crate::{
  models::{
    multi_dimen_point::MultiDimenPoint
  }
};

pub struct LineSegment<'a> {
  start_point: &'a MultiDimenPoint,
  end_point: &'a MultiDimenPoint,
  trajectory_id: usize,
  order: usize
}

impl<'a> LineSegment<'a> {
  pub fn new(trajectory_id: usize, order: usize, start_point: &'a MultiDimenPoint, end_point: &'a MultiDimenPoint) -> Self {
    Self {
      start_point,
      end_point,
      trajectory_id,
      order
    }
  }

  /// 根据指定的线段获取起点与终点
  pub fn extract_start_end_points(&self) -> (&MultiDimenPoint, &MultiDimenPoint) {
    (self.start_point, self.end_point)
  }

  pub fn get_trajectory_id(&self) -> usize {
    self.trajectory_id
  }
}