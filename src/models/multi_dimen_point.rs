pub struct MultiDimenPoint {
  dimension: usize,
  coordinate: Vec<f64>
}

impl MultiDimenPoint {
  /// 根据维度创建一个多维点
  pub fn new(dimension: usize) -> Self {
    // 初始化 coordinate
    let mut coordinate = Vec::with_capacity(dimension);
    for _ in 0..dimension {
      coordinate.push(0.0);
    }

    Self {
      dimension,
      coordinate
    }
  }

  /// 返回该点的维度
  pub fn get_dimension(&self) -> usize {
    self.dimension
  }

  /// 返回第 n 个坐标
  /// 
  /// 注意：由于 nth 可能大于维度，所以需要返回 Option
  pub fn get_nth_coordinate(&self, nth: usize) -> Option<&f64> {
    self.coordinate.get(nth)
  }

  /// 设置第 n 个坐标为指定值
  /// 
  /// 注意： 由于 nth 可能大于维度，所以需要知道是否成功设置
  pub fn set_nth_coordinate(&mut self, nth: usize, value: f64) -> bool {
    if nth >= self.dimension { return false; }
    self.coordinate[nth] = value;

    true
  }
}