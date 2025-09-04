#[derive(Debug)]
/// `scores` - unsorted vector
/// `latest_score` - last element of the vector
pub struct HighScores {
    scores: Vec<u32>,
    latest_score: Option<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            scores: scores.to_vec(),
            latest_score: scores.last().copied(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.latest_score
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().copied().max()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted = self.scores.clone();
        sorted.sort_by(|a, b| b.cmp(a));
        sorted.truncate(3);
        sorted
    }
}
