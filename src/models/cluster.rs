use crate::{
  models::{
    multi_dimen_point::MultiDimenPoint,
    model_error::ModelError
  }
};

pub struct Cluster {
  id: usize,
  dimension: usize,
  density: usize,
  points: Vec<MultiDimenPoint>
}

impl Cluster {
  pub fn new(id: usize, dimension: usize) -> Self {
    Self {
      id,
      dimension,
      density: 0,
      points: Vec::new()
    }
  }

  /// 获得簇的 id
  pub fn get_id(&self) -> usize {
    self.id
  }

  /// 设置簇的密度
  /// 其实就是簇内含有的轨迹数量
  pub fn set_density(&mut self, density: usize) {
    self.density = density;
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