#[derive(Debug)]
pub struct HighScores {
  scores: Vec<u32>,
}

impl HighScores {
  pub fn new(scores: &[u32]) -> Self {
    HighScores {
      scores: scores.iter().map(|&x| x).collect(),
    }
  }

  pub fn scores(&self) -> &[u32] {
    &self.scores
  }

  pub fn latest(&self) -> Option<u32> {
    self.scores.last().map(|&x| x)
  }

  pub fn personal_best(&self) -> Option<u32> {
    self.scores.iter().max().map(|&x| x)
  }

  pub fn personal_top_three(&self) -> Vec<u32> {
    let mut result: Vec<_> = self.scores.iter().map(|&x| x).collect();
    result.sort_by(|lhs, rhs| rhs.cmp(lhs));
    result.iter().take(3).map(|&x| x).collect()
  }
}
