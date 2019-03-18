use crate::{
  models::{
    multi_dimen_point::MultiDimenPoint
  }
};

pub struct LineSegment {
  line_segment: MultiDimenPoint,
  trajectory_id: usize,
  order: usize
}

impl LineSegment {
  pub fn new(dimension: usize, trajectory_id: usize, order: usize) -> Self {
    Self {
      line_segment: MultiDimenPoint::new(dimension),
      trajectory_id,
      order
    }
  }

  /// 获得只读的线段
  pub fn get_line_segment(&self) -> &MultiDimenPoint {
    &self.line_segment
  }

  /// 获得可写的线段
  pub fn get_mut_line_segment(&mut self) -> &mut MultiDimenPoint {
    &mut self.line_segment
  }
}