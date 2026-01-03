use std::clone::Clone;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// 项目包含分数和路径
#[derive(Debug, Clone)]
pub struct Item {
    score: f64,
    path: Vec<String>,
}

impl Item {
    pub fn new(score: f64, path: Vec<String>) -> Self {
        Item { score, path }
    }

    pub fn score(&self) -> f64 {
        self.score
    }

    pub fn path(&self) -> &Vec<String> {
        &self.path
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Item {}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .score
            .partial_cmp(&self.score)
            .unwrap_or(Ordering::Equal)
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "< score={}, path={:?} >", self.score, self.path)
    }
}

/// 优先集合，保持最多capacity个元素
#[derive(Clone)]
pub struct PrioritySet {
    capacity: usize,
    data: BinaryHeap<Item>,
}

impl PrioritySet {
    pub fn new(capacity: usize) -> Self {
        PrioritySet {
            capacity,
            data: BinaryHeap::new(),
        }
    }

    pub fn put(&mut self, score: f64, path: Vec<String>) {
        let item = Item::new(score, path);
        self.data.push(item);

        // 保持容量限制
        while self.len() > self.capacity {
            // 弹出最小的元素，BinaryHeap 是最大堆，但 Item 实现了反转的比较
            self.data.pop();
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Item> {
        self.data.iter()
    }

    /// 转换为排序的向量（按分数降序）
    pub fn to_sorted_vec(&self) -> Vec<Item> {
        let mut items: Vec<Item> = self.data.iter().cloned().collect();
        items.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal));
        items
    }
}

impl std::fmt::Display for PrioritySet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[")?;
        for item in &self.data {
            writeln!(f, "\t{}", item)?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_set() {
        let mut ps = PrioritySet::new(3);
        ps.put(0.5, vec!["你".to_string()]);
        ps.put(0.8, vec!["我".to_string()]);
        ps.put(0.3, vec!["他".to_string()]);
        // 应该替换掉0.3的元素
        ps.put(0.9, vec!["她".to_string()]);

        assert_eq!(ps.len(), 3);

        let sorted_items = ps.to_sorted_vec();
        assert_eq!(sorted_items[0].score, 0.9);
        assert_eq!(sorted_items[0].path, vec!["她".to_string()]);
    }
}
