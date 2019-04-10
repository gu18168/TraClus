use crate::{
  models::{
    point::Point
  }
};
use uuid::Uuid;
use std::hash::{Hash, Hasher};

pub struct LineSegment<'a> {
  uuid: Uuid,
  start_point: &'a Point,
  end_point: &'a Point,
  trajectory_id: usize
}

impl<'a> PartialEq for LineSegment<'a> {
  fn eq(&self, other: &LineSegment) -> bool {
    self.uuid == other.uuid
  }
}

impl<'a> Eq for LineSegment<'a> {}

impl<'a> Hash for LineSegment<'a> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.uuid.hash(state);
  }
}

impl<'a> LineSegment<'a> {
  pub fn new(trajectory_id: usize, start_point: &'a Point, end_point: &'a Point) -> Self {
    Self {
      uuid: Uuid::new_v4(),
      start_point,
      end_point,
      trajectory_id
    }
  }

  pub fn get_uuid(&self) -> &Uuid {
    &self.uuid
  }

  /// 根据指定的线段获取起点与终点
  pub fn extract_start_end_points(&self) -> (&Point, &Point) {
    (self.start_point, self.end_point)
  }

  pub fn get_trajectory_id(&self) -> usize {
    self.trajectory_id
  }
}