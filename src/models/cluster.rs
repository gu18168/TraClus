use crate::{
  models::{
    multi_dimen_point::MultiDimenPoint,
    model_error::ModelError
  }
};

pub struct Cluster {
  id: usize,
  dimension: usize,
  points: Vec<MultiDimenPoint>
}

impl Cluster {
  pub fn new(id: usize, dimension: usize, points: Vec<MultiDimenPoint>) -> Self {
    Self {
      id,
      dimension,
      points
    }
  }

  /// 获得簇的 id
  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn get_len(&self) -> usize {
    self.points.len()
  }

  pub fn get_points(&self) -> &Vec<MultiDimenPoint> {
    &self.points
  }

  /// 簇内增加点
  /// 
  /// 注意：若添加的点维度与簇维度不同将会返回 Err
  pub fn add_point(&mut self, point: MultiDimenPoint) -> Result<(), ModelError> {
    if point.get_dimension() != self.dimension { return Err(ModelError::MismatchDimension); }

    self.points.push(point);
    Ok(())
  }
}