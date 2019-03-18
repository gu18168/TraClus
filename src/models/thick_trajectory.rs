use crate::{
  models::{
    multi_dimen_point::MultiDimenPoint
  }
};

pub struct ThickTrajectory {
  id: usize,
  dimension: usize,
  partition_points: Vec<MultiDimenPoint>
}

impl ThickTrajectory {
  pub fn new(id: usize, dimension: usize, partition_points: Vec<MultiDimenPoint>) -> Self {
    Self {
      id,
      dimension,
      partition_points
    }
  }

  pub fn get_len(&self) -> usize {
    self.partition_points.len()
  }

  pub fn get_dimension(&self) -> usize {
    self.dimension
  }

  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn get_partition_point(&self, index: usize) -> Option<&MultiDimenPoint> {
    self.partition_points.get(index)
  }
}