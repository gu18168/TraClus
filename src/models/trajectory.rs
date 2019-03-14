use crate::{
  models::{
    multi_dimen_point::MultiDimenPoint,
    model_error::ModelError
  }
};

pub struct Trajectory {
  id: usize,
  dimension: usize,
  points: Vec<MultiDimenPoint>,
  partition_point: Vec<MultiDimenPoint>
}

impl Trajectory {
  pub fn new(id: usize, dimension: usize) -> Self {
    Self {
      id,
      dimension,
      points: Vec::new(),
      partition_point: Vec::new()
    }
  }

  /// 获得轨迹的 id
  pub fn get_id(&self) -> usize {
    self.id
  }

  /// 在簇中添加轨迹点
  /// 
  /// 注意：若添加的点维度与轨迹维度不同将会返回 Err
  pub fn add_point(&mut self, point: MultiDimenPoint) -> Result<(), ModelError> {
    if point.get_dimension() != self.dimension { return Err(ModelError::MismatchDimension); }

    self.points.push(point);
    Ok(())
  }

  /// 在簇中添加划分点
  /// 
  /// 注意：若添加的点维度与轨迹维度不同将会返回 Err
  pub fn add_partition_point(&mut self, point: MultiDimenPoint) -> Result<(), ModelError> {
    if point.get_dimension() != self.dimension { return Err(ModelError::MismatchDimension); }

    self.partition_point.push(point);
    Ok(())
  }
}