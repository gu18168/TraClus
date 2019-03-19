use crate::{
  models::{
    line_segment::LineSegment,
    multi_dimen_point::MultiDimenPoint
  },
  distance_util::{
    measure_distance_line_to_line
  }
};

static UNCLASSIFIED: i32 = -2;
static NOISE: i32 = -1;

pub fn perform_dbscan(eps: f64, min_lns: usize, line_segments: &Vec<LineSegment>) -> (Vec<i32>, usize) {
  // 这个变量其实起到的是 F(a) -> b 的作用
  // a 是目前的线段索引，b 是该线段的簇索引
  let mut cluster_indexs: Vec<i32> = Vec::new();
  let mut cluster_index: usize = 0;

  // 所有的线段初始化
  let len = line_segments.len();
  for _ in 0..len {
    cluster_indexs.push(UNCLASSIFIED);
  }

  for i in 0..len {
    if *cluster_indexs.get(i).unwrap() == UNCLASSIFIED
      && expand_cluster(i, cluster_index, eps, min_lns, line_segments, &mut cluster_indexs)
    {
      cluster_index += 1;
    }
  }

  (cluster_indexs, cluster_index)
}

// 感觉是可以优化的
fn expand_cluster(index: usize, cluster_index: usize, eps: f64, min_lns: usize,
  line_segments: &Vec<LineSegment>, cluster_indexs: &mut Vec<i32>) -> bool 
{
  let (line_1_start, line_1_end) = line_segments.get(index).unwrap().extract_start_end_points();
  let mut seeds = compute_eps_neighborhood(eps, line_1_start, line_1_end, line_segments);

  let len = seeds.len();
  if len < min_lns {
    cluster_indexs[index] = NOISE;
    return false;
  }

  for i in 0..len {
    cluster_indexs[seeds[i]] = cluster_index as i32;
  }

  let mut index = 0;
  while index < seeds.len() {
    let seed = seeds[index];
    let (line_1_start, line_1_end) = line_segments.get(seed).unwrap().extract_start_end_points();
    let result_seeds = compute_eps_neighborhood(eps, line_1_start, line_1_end, line_segments);

    if result_seeds.len() >= min_lns {
      for result_seed in result_seeds {
        let temp_index = cluster_indexs[result_seed];
        if temp_index < 0 {
          if temp_index == UNCLASSIFIED {
            seeds.push(temp_index as usize);
          }
          cluster_indexs[result_seed] = cluster_index as i32;
        }
      }
    }

    index += 1;
  }

  true
}

/// 计算一条线段的 eps 邻居集
fn compute_eps_neighborhood(
  eps: f64,
  line_1_start: &MultiDimenPoint, 
  line_1_end: &MultiDimenPoint, 
  line_segments: &Vec<LineSegment>) -> Vec<usize> 
{
  let mut result = Vec::new();

  for i in 0..line_segments.len() {
    let (line_2_start, line_2_end) = line_segments.get(i).unwrap().extract_start_end_points();
    let distance = measure_distance_line_to_line(line_1_start, line_1_end, line_2_start, line_2_end);

    if distance < eps { result.push(i); }
  }

  result
}