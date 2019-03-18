use crate::{
  models::{
    multi_dimen_point::MultiDimenPoint,
    model_error::ModelError
  }
};

pub struct Trajectory {
  id: usize,
  dimension: usize,
  points: Vec<MultiDimenPoint>
}

impl Trajectory {
  pub fn new(id: usize, dimension: usize) -> Self {
    Self {
      id,
      dimension,
      points: Vec::new()
    }
  }

  /// 获得轨迹的 id
  pub fn get_id(&self) -> usize {
    self.id
  }

  /// 获得轨迹的维度
  pub fn get_dimension(&self) -> usize {
    self.dimension
  }

  /// 在簇中添加轨迹点
  /// 
  /// 注意：若添加的点维度与轨迹维度不同将会返回 Err
  pub fn add_point(&mut self, point: MultiDimenPoint) -> Result<(), ModelError> {
    if point.get_dimension() != self.dimension { return Err(ModelError::MismatchDimension); }

    self.points.push(point);
    Ok(())
  }

  pub fn get_points(self) -> Vec<MultiDimenPoint> {
    self.points
  }

  /// 获取指定索引的轨迹点
  pub fn get_point(&self, index: usize) -> Option<&MultiDimenPoint> {
    self.points.get(index)
  }

  /// 获取轨迹点集的长度
  pub fn get_points_len(&self) -> usize {
    self.points.len()
  }
}