use crate::types::{TrackPoint, AbstractKNN, Neighbor};
use std::collections::BinaryHeap;


/// This is a node within a KD Tree, see the documentation on `KDTree` for more information.
#[derive(Clone)]
pub struct KDTreeNode {
    point: TrackPoint,
    left: Option<Box<KDTreeNode>>,
    right: Option<Box<KDTreeNode>>
}

/// This is an implementation of a K-Dimensional (K-D) Binary Tree.
/// It has the following time complexities (n is the number of nodes in the tree):
/// * Insertion: Average: O(log(n)), Worst: O(n)
/// * Search: Average: O(log(n)), Worst: O(n)
/// * Nearest Neighbors: Average: O(log(n)), Worst: O(n)
///     * This is because the nearest neighbors function simply performs a search operation O(log(n)) then branches downwards from it.
///     * We also can ignore the sorting by distance as it is a heap sort of O(klog(n)) which is negligible with reasonable k values.
pub struct KDTree {
    root: Option<Box<KDTreeNode>>,
    ignore: Vec<String>,
    size: usize
}

impl KDTree {
    pub fn new() -> Self {
        KDTree { root: None, size: 0, ignore: Vec::new() }
    }

    // Inserts a point using recursion into a node.
    fn insert_node(&self, node: Option<Box<KDTreeNode>>, point: TrackPoint, depth: usize) -> KDTreeNode {
        match node {
            None => KDTreeNode {
                point,
                left: None,
                right: None,
            },
            Some(mut n) => {
                // Recursively do a binary tree insertion, swapping dimensions each layer.
                let cd = depth % 2;
                if point.cd_to_float(cd) < n.point.cd_to_float(cd) {
                    n.left = Some(Box::new(self.insert_node(n.left.take(), point, depth + 1)));
                } else {
                    n.right = Some(Box::new(self.insert_node(n.right.take(), point, depth + 1)));
                }
                *n
            }
        }
    }

    pub fn create_in_place(&mut self, mut points: Vec<TrackPoint>, depth: usize) {
        self.size = points.len();
        let mut points = points.as_mut_slice();
        self.root = Self::create_in_place_rec(points, depth)
    }

    // Inserts a point using recursion into a node.
    fn create_in_place_rec(points: &mut [TrackPoint], depth: usize) -> Option<Box<KDTreeNode>> {
        if points.is_empty() {
            return None;
        }
    
        // Sort points based on the current dimension
        points.sort_by(|a, b| a.cd_to_float(depth % 2).partial_cmp(&b.cd_to_float(depth % 2)).unwrap());
    
        // Find the median
        let median = points.len() / 2;
    
        // Create a new node for the median point
        let mut node = KDTreeNode {
            point: points[median].clone(),
            left: None,
            right: None,
        };
    
        // Recursively build the left and right subtrees
        if median > 0 {
            node.left = Self::create_in_place_rec(&mut points[..median], depth + 1);
        }
        if median + 1 < points.len() {
            node.right = Self::create_in_place_rec(&mut points[median + 1..], depth + 1);
        }
    
        Some(Box::new(node))
    }

    // Recursively iterates through the tree searching for nearest neighbors
    fn nearest_neighbors_rec(&self, node: &Option<Box<KDTreeNode>>, point: &TrackPoint, depth: usize, k: usize, heap: &mut BinaryHeap<Neighbor>) {
        if let Some(n) = node {
            let distance = self.distance(&n.point, point);
            // Constantly push and pop off of the heap to create a rolling window of points, only keeping the smallest distance neighbors.
            if heap.len() < k || distance < heap.peek().unwrap().distance {
                // Pop off this largest distance value to fit a lower distance value.
                if !self.ignore.contains(&n.point.id) {
                    if heap.len() == k {
                        heap.pop();
                    }
                
                    heap.push(Neighbor { distance, point: n.point.clone() });
                }
            }

            // Determine what the next branch should be by figuring out whether the current node is smaller or larger than the reference point.
            let cd = depth % 2;
            let next_branch = if point.cd_to_float(cd) < n.point.cd_to_float(cd) { &n.left } else { &n.right };
            let opposite_branch = if next_branch.is_some() { &n.right } else { &n.left };

            // Recurse
            self.nearest_neighbors_rec(next_branch, point, depth + 1, k, heap);
            // Recurse opposite if the heap is not full yet or the current node is ridiculously far away in a single dimension.
            if heap.len() < k || (point.cd_to_float(cd) - n.point.cd_to_float(cd)).abs() < heap.peek().unwrap().distance {
                self.nearest_neighbors_rec(opposite_branch, point, depth + 1, k, heap);
            }
        }
    }

    // Euclidean distance formula
    fn distance(&self, point1: &TrackPoint, point2: &TrackPoint) -> f64 {
        ((point2.x - point1.x).powi(2) + (point2.y - point1.y).powi(2)).sqrt()
    }
}

impl AbstractKNN for KDTree {
    fn insert(&mut self, point: TrackPoint) {
        self.size += 1;
        self.root  = Some(Box::new(self.insert_node(self.root.clone(), point, 0)));
    }

    fn size(&self) -> usize {
        return self.size
    }
     
    fn nearest_neighbors(&self, point: &TrackPoint, k: usize) -> Vec<TrackPoint> {
        let mut heap = BinaryHeap::new();
        self.nearest_neighbors_rec(&self.root, point, 0, k, &mut heap);
        heap.into_sorted_vec().into_iter().map(|n| n.point).collect()
    }

    fn add_ignore(&mut self, id: String) {
        self.ignore.push(id);
    }

    fn ignore_size(&self) -> usize {
        self.ignore.len()
    }

    fn pop_ignore(&mut self) {
        self.ignore.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{TrackPoint, AbstractKNN};

    use super::KDTree;

    #[test]
    fn test_map_creation() {
        let mut map = KDTree::new();

        map.insert(TrackPoint{
            x: 12.0,
            y: 6.0,
            duration_ms: 1000,
            explicit: false,
            name: "".into(),
            id: "test1".into(),
            artists: Vec::new(),
        });

        map.insert(TrackPoint{
            x: 10.0,
            y: 6.0,
            duration_ms: 1000,
            explicit: false,
            name: "".into(),
            id: "test2".into(),
            artists: Vec::new(),
        });

        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_aknn() {
        let mut map = KDTree::new();

        map.insert(TrackPoint{
            x: 12.0,
            y: 6.0,
            duration_ms: 1000,
            explicit: false,
            name: "".into(),
            id: "test1".into(),
            artists: Vec::new(),
        });

        map.insert(TrackPoint{
            x: 10.0,
            y: 6.0,
            duration_ms: 1000,
            explicit: false,
            name: "".into(),
            id: "test2".into(),
            artists: Vec::new(),
        });

        map.insert(TrackPoint{
            x: 24.0,
            y: 6.0,
            duration_ms: 1000,
            explicit: false,
            name: "".into(),
            id: "test3".into(),
            artists: Vec::new(),
        });

        assert_eq!(map.size(), 3);

        let out = map.nearest_neighbors(&TrackPoint{
            x: 10.0,
            y: 6.0,
            duration_ms: 1000,
            explicit: false,
            name: "".into(),
            id: "test3".into(),
            artists: Vec::new(),
        }, 2);

        assert_eq!(out.len(), 2);
    }
}