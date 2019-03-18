use crate::{
  models::{
    multi_dimen_point::MultiDimenPoint,
    model_error::ModelError
  }
};

pub struct Trajectory<'a> {
  id: usize,
  dimension: usize,
  points: Vec<MultiDimenPoint>,
  partition_point: Vec<&'a MultiDimenPoint>
}

impl<'a> Trajectory<'a> {
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
  /// 因为划分点其实就是特殊的轨迹点，所以只用引用到轨迹点即可
  /// 注意：这里将指定索引的轨迹点引用加入到划分点中
  pub fn add_partition_point(&'a mut self, index: usize) -> Result<(), ModelError> {
    if index >= self.points.len() { return Err(ModelError::MismatchIndex); }

    self.partition_point.push(self.points.get(index).unwrap());
    Ok(())
  }

  pub fn add_partition_points(&'a mut self, indexs: Vec<usize>) {
    for index in indexs {
      self.partition_point.push(self.points.get(index).unwrap());
    }
  }

  /// 获取轨迹点集的读引用
  pub fn get_points(&self) -> &Vec<MultiDimenPoint> {
    &self.points
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