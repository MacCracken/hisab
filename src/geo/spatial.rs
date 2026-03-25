use super::*;

// BVH (Bounding Volume Hierarchy)
// ---------------------------------------------------------------------------

/// A node in a BVH tree.
#[derive(Debug, Clone)]
enum BvhNode {
    Leaf {
        bounds: Aabb,
        index: usize,
    },
    Internal {
        bounds: Aabb,
        left: Box<BvhNode>,
        right: Box<BvhNode>,
    },
}

/// A Bounding Volume Hierarchy for fast ray-AABB broadphase queries.
///
/// Stores a tree of AABBs built from a list of primitives. Each primitive
/// is represented by its AABB and an index into the caller's primitive array.
#[derive(Debug, Clone)]
#[must_use]
pub struct Bvh {
    root: Option<BvhNode>,
    len: usize,
}

impl Bvh {
    /// Build a BVH from a list of (aabb, index) pairs using midpoint splitting.
    pub fn build(items: &mut [(Aabb, usize)]) -> Self {
        let len = items.len();
        if items.is_empty() {
            return Self { root: None, len: 0 };
        }
        let root = Self::build_recursive(items);
        Self {
            root: Some(root),
            len,
        }
    }

    fn build_recursive(items: &mut [(Aabb, usize)]) -> BvhNode {
        if items.len() == 1 {
            return BvhNode::Leaf {
                bounds: items[0].0,
                index: items[0].1,
            };
        }

        // Compute bounding box of all items
        let mut bounds = items[0].0;
        for item in items.iter().skip(1) {
            bounds = bounds.merge(&item.0);
        }

        // Split along the longest axis at the midpoint
        let size = bounds.size();
        let axis = if size.x >= size.y && size.x >= size.z {
            0
        } else if size.y >= size.z {
            1
        } else {
            2
        };

        let mid = match axis {
            0 => (bounds.min.x + bounds.max.x) * 0.5,
            1 => (bounds.min.y + bounds.max.y) * 0.5,
            _ => (bounds.min.z + bounds.max.z) * 0.5,
        };

        // Partition items around the midpoint (O(n) instead of O(n log n) sort)
        let center_val = |aabb: &Aabb| -> f32 {
            let arr = aabb.center().to_array();
            arr[axis]
        };

        // In-place partition: items with center < mid go left
        let mut split = 0;
        for i in 0..items.len() {
            if center_val(&items[i].0) < mid {
                items.swap(i, split);
                split += 1;
            }
        }
        // Ensure at least one item on each side
        if split == 0 {
            split = 1;
        } else if split == items.len() {
            split = items.len() - 1;
        }

        let (left_items, right_items) = items.split_at_mut(split);
        let left = Self::build_recursive(left_items);
        let right = Self::build_recursive(right_items);

        BvhNode::Internal {
            bounds,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Number of primitives in the BVH.
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the BVH is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Query all primitive indices whose AABBs are hit by the ray.
    ///
    /// Returns indices in no particular order. Use the indices to test
    /// against the actual primitives for exact intersection.
    #[must_use]
    pub fn query_ray(&self, ray: &Ray) -> Vec<usize> {
        let mut results = Vec::new();
        if let Some(ref root) = self.root {
            Self::query_ray_recursive(root, ray, &mut results);
        }
        results
    }

    fn query_ray_recursive(node: &BvhNode, ray: &Ray, results: &mut Vec<usize>) {
        match node {
            BvhNode::Leaf { bounds, index } => {
                if ray_aabb(ray, bounds).is_some() {
                    results.push(*index);
                }
            }
            BvhNode::Internal {
                bounds,
                left,
                right,
            } => {
                if ray_aabb(ray, bounds).is_some() {
                    Self::query_ray_recursive(left, ray, results);
                    Self::query_ray_recursive(right, ray, results);
                }
            }
        }
    }

    /// Query all primitive indices whose AABBs overlap the given AABB.
    #[must_use]
    pub fn query_aabb(&self, query: &Aabb) -> Vec<usize> {
        let mut results = Vec::new();
        if let Some(ref root) = self.root {
            Self::query_aabb_recursive(root, query, &mut results);
        }
        results
    }

    fn query_aabb_recursive(node: &BvhNode, query: &Aabb, results: &mut Vec<usize>) {
        match node {
            BvhNode::Leaf { bounds, index } => {
                if aabb_aabb(bounds, query) {
                    results.push(*index);
                }
            }
            BvhNode::Internal {
                bounds,
                left,
                right,
            } => {
                if aabb_aabb(bounds, query) {
                    Self::query_aabb_recursive(left, query, results);
                    Self::query_aabb_recursive(right, query, results);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// K-d tree
// ---------------------------------------------------------------------------

/// A node in a k-d tree.
#[derive(Debug, Clone)]
enum KdNode {
    Leaf {
        point: Vec3,
        index: usize,
    },
    Split {
        axis: usize,
        split_val: f32,
        left: Box<KdNode>,
        right: Box<KdNode>,
    },
}

/// A 3D k-d tree for nearest-neighbor point queries.
#[derive(Debug, Clone)]
#[must_use]
pub struct KdTree {
    root: Option<KdNode>,
    len: usize,
}

impl KdTree {
    /// Build a k-d tree from a list of (point, index) pairs.
    pub fn build(items: &mut [(Vec3, usize)]) -> Self {
        let len = items.len();
        if items.is_empty() {
            return Self { root: None, len: 0 };
        }
        let root = Self::build_recursive(items, 0);
        Self {
            root: Some(root),
            len,
        }
    }

    fn build_recursive(items: &mut [(Vec3, usize)], depth: usize) -> KdNode {
        if items.len() == 1 {
            return KdNode::Leaf {
                point: items[0].0,
                index: items[0].1,
            };
        }

        let axis = depth % 3;
        let mid = items.len() / 2;
        // O(n) partial sort: only ensures the median element is in position
        items.select_nth_unstable_by(mid, |a, b| {
            a.0.to_array()[axis]
                .partial_cmp(&b.0.to_array()[axis])
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let split_val = items[mid].0.to_array()[axis];

        let (left_items, right_items) = items.split_at_mut(mid);
        let left = Self::build_recursive(left_items, depth + 1);
        let right = Self::build_recursive(right_items, depth + 1);

        KdNode::Split {
            axis,
            split_val,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Number of points in the tree.
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the tree is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Find the nearest neighbor to `query`.
    ///
    /// Returns `(index, distance_squared)` of the closest point,
    /// or `None` if the tree is empty.
    #[must_use]
    pub fn nearest(&self, query: Vec3) -> Option<(usize, f32)> {
        let mut best_idx = 0;
        let mut best_dist_sq = f32::INFINITY;
        if let Some(ref root) = self.root {
            Self::nearest_recursive(root, query, &mut best_idx, &mut best_dist_sq);
        }
        if best_dist_sq.is_finite() {
            Some((best_idx, best_dist_sq))
        } else {
            None
        }
    }

    fn nearest_recursive(node: &KdNode, query: Vec3, best_idx: &mut usize, best_dist_sq: &mut f32) {
        match node {
            KdNode::Leaf { point, index } => {
                let dist_sq = (*point - query).length_squared();
                if dist_sq < *best_dist_sq {
                    *best_dist_sq = dist_sq;
                    *best_idx = *index;
                }
            }
            KdNode::Split {
                axis,
                split_val,
                left,
                right,
            } => {
                let query_val = query.to_array()[*axis];
                let diff = query_val - split_val;

                // Visit the closer side first
                let (near, far) = if diff < 0.0 {
                    (left.as_ref(), right.as_ref())
                } else {
                    (right.as_ref(), left.as_ref())
                };

                Self::nearest_recursive(near, query, best_idx, best_dist_sq);

                // Only visit the far side if the splitting plane is closer than current best
                if diff * diff < *best_dist_sq {
                    Self::nearest_recursive(far, query, best_idx, best_dist_sq);
                }
            }
        }
    }

    /// Find all points within `radius` of `query`.
    ///
    /// Returns a list of `(index, distance_squared)`.
    #[must_use]
    pub fn within_radius(&self, query: Vec3, radius: f32) -> Vec<(usize, f32)> {
        let mut results = Vec::new();
        let radius_sq = radius * radius;
        if let Some(ref root) = self.root {
            Self::radius_recursive(root, query, radius_sq, &mut results);
        }
        results
    }

    fn radius_recursive(
        node: &KdNode,
        query: Vec3,
        radius_sq: f32,
        results: &mut Vec<(usize, f32)>,
    ) {
        match node {
            KdNode::Leaf { point, index } => {
                let dist_sq = (*point - query).length_squared();
                if dist_sq <= radius_sq {
                    results.push((*index, dist_sq));
                }
            }
            KdNode::Split {
                axis,
                split_val,
                left,
                right,
            } => {
                let query_val = query.to_array()[*axis];
                let diff = query_val - split_val;

                let (near, far) = if diff < 0.0 {
                    (left.as_ref(), right.as_ref())
                } else {
                    (right.as_ref(), left.as_ref())
                };

                Self::radius_recursive(near, query, radius_sq, results);

                if diff * diff <= radius_sq {
                    Self::radius_recursive(far, query, radius_sq, results);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Quadtree (2D spatial index)
// ---------------------------------------------------------------------------

/// A 2D axis-aligned bounding rectangle for quadtree operations.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub min: glam::Vec2,
    pub max: glam::Vec2,
}

impl Rect {
    /// Create a new rectangle. Min/max are corrected if swapped.
    #[must_use]
    #[inline]
    pub fn new(a: glam::Vec2, b: glam::Vec2) -> Self {
        Self {
            min: a.min(b),
            max: a.max(b),
        }
    }

    /// Check whether a 2D point is inside this rectangle.
    #[must_use]
    #[inline]
    pub fn contains_point(&self, p: glam::Vec2) -> bool {
        p.cmpge(self.min).all() && p.cmple(self.max).all()
    }

    /// Check whether two rectangles overlap.
    #[must_use]
    #[inline]
    pub fn overlaps(&self, other: &Rect) -> bool {
        self.min.cmple(other.max).all() && other.min.cmple(self.max).all()
    }

    /// Center of the rectangle.
    #[must_use]
    #[inline]
    pub fn center(&self) -> glam::Vec2 {
        (self.min + self.max) * 0.5
    }

    /// Size (extents) of the rectangle.
    #[must_use]
    #[inline]
    pub fn size(&self) -> glam::Vec2 {
        self.max - self.min
    }

    /// Merge two rectangles into one that encloses both.
    #[must_use]
    #[inline]
    pub fn merge(&self, other: &Rect) -> Rect {
        Rect {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    /// Area of the rectangle.
    #[must_use]
    #[inline]
    pub fn area(&self) -> f32 {
        let s = self.size();
        s.x * s.y
    }
}

/// A quadtree node.
#[derive(Debug, Clone)]
enum QuadNode {
    Empty,
    Leaf(Vec<(glam::Vec2, usize)>),
    Split {
        children: Box<[QuadNode; 4]>, // NW, NE, SW, SE
        bounds: Rect,
    },
}

/// A 2D quadtree for spatial point queries.
///
/// Points are inserted into a fixed-bounds region. The tree subdivides
/// when a leaf exceeds `max_per_leaf` items, up to `max_depth` levels.
#[derive(Debug, Clone)]
pub struct Quadtree {
    root: QuadNode,
    bounds: Rect,
    len: usize,
    max_per_leaf: usize,
    max_depth: usize,
}

impl Quadtree {
    /// Create a new empty quadtree covering the given bounds.
    ///
    /// `max_per_leaf`: max items before a leaf splits (default 8).
    /// `max_depth`: max subdivision depth (default 8).
    #[must_use]
    pub fn new(bounds: Rect, max_per_leaf: usize, max_depth: usize) -> Self {
        Self {
            root: QuadNode::Empty,
            bounds,
            len: 0,
            max_per_leaf,
            max_depth,
        }
    }

    /// Number of items in the quadtree.
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the quadtree is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Insert a point with an associated index.
    pub fn insert(&mut self, point: glam::Vec2, index: usize) {
        if !self.bounds.contains_point(point) {
            return; // Out of bounds, ignore
        }
        Self::insert_recursive(
            &mut self.root,
            point,
            index,
            &self.bounds,
            self.max_per_leaf,
            self.max_depth,
            0,
        );
        self.len += 1;
    }

    fn insert_recursive(
        node: &mut QuadNode,
        point: glam::Vec2,
        index: usize,
        bounds: &Rect,
        max_per_leaf: usize,
        max_depth: usize,
        depth: usize,
    ) {
        match node {
            QuadNode::Empty => {
                *node = QuadNode::Leaf(vec![(point, index)]);
            }
            QuadNode::Leaf(items) => {
                items.push((point, index));
                if items.len() > max_per_leaf && depth < max_depth {
                    // Split
                    let center = bounds.center();
                    let mut children = Box::new([
                        QuadNode::Empty,
                        QuadNode::Empty,
                        QuadNode::Empty,
                        QuadNode::Empty,
                    ]);
                    for &(p, idx) in items.iter() {
                        let quadrant = Self::quadrant(p, center);
                        let child_bounds = Self::child_bounds(bounds, quadrant);
                        Self::insert_recursive(
                            &mut children[quadrant],
                            p,
                            idx,
                            &child_bounds,
                            max_per_leaf,
                            max_depth,
                            depth + 1,
                        );
                    }
                    *node = QuadNode::Split {
                        children,
                        bounds: *bounds,
                    };
                }
            }
            QuadNode::Split {
                children,
                bounds: node_bounds,
            } => {
                let center = node_bounds.center();
                let quadrant = Self::quadrant(point, center);
                let child_bounds = Self::child_bounds(node_bounds, quadrant);
                Self::insert_recursive(
                    &mut children[quadrant],
                    point,
                    index,
                    &child_bounds,
                    max_per_leaf,
                    max_depth,
                    depth + 1,
                );
            }
        }
    }

    fn quadrant(point: glam::Vec2, center: glam::Vec2) -> usize {
        let x = if point.x >= center.x { 1 } else { 0 };
        let y = if point.y >= center.y { 0 } else { 2 }; // NW=0, NE=1, SW=2, SE=3
        x + y
    }

    fn child_bounds(bounds: &Rect, quadrant: usize) -> Rect {
        let center = bounds.center();
        match quadrant {
            0 => Rect::new(
                glam::Vec2::new(bounds.min.x, center.y),
                glam::Vec2::new(center.x, bounds.max.y),
            ), // NW
            1 => Rect::new(center, bounds.max), // NE
            2 => Rect::new(bounds.min, center), // SW
            _ => Rect::new(
                glam::Vec2::new(center.x, bounds.min.y),
                glam::Vec2::new(bounds.max.x, center.y),
            ), // SE
        }
    }

    /// Query all indices of points within the given rectangle.
    #[must_use]
    pub fn query_rect(&self, query: &Rect) -> Vec<usize> {
        let mut results = Vec::new();
        Self::query_recursive(&self.root, &self.bounds, query, &mut results);
        results
    }

    fn query_recursive(node: &QuadNode, bounds: &Rect, query: &Rect, results: &mut Vec<usize>) {
        if !bounds.overlaps(query) {
            return;
        }
        match node {
            QuadNode::Empty => {}
            QuadNode::Leaf(items) => {
                for &(p, idx) in items {
                    if query.contains_point(p) {
                        results.push(idx);
                    }
                }
            }
            QuadNode::Split {
                children,
                bounds: node_bounds,
            } => {
                for q in 0..4 {
                    let child_bounds = Self::child_bounds(node_bounds, q);
                    Self::query_recursive(&children[q], &child_bounds, query, results);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Octree (3D spatial index)
// ---------------------------------------------------------------------------

/// An octree node.
#[derive(Debug, Clone)]
enum OctNode {
    Empty,
    Leaf(Vec<(Vec3, usize)>),
    Split {
        children: Box<[OctNode; 8]>,
        bounds: Aabb,
    },
}

/// A 3D octree for spatial point queries.
///
/// Same design as [`Quadtree`] but in 3D with 8 children per split.
#[derive(Debug, Clone)]
pub struct Octree {
    root: OctNode,
    bounds: Aabb,
    len: usize,
    max_per_leaf: usize,
    max_depth: usize,
}

impl Octree {
    /// Create a new empty octree covering the given bounds.
    #[must_use]
    pub fn new(bounds: Aabb, max_per_leaf: usize, max_depth: usize) -> Self {
        Self {
            root: OctNode::Empty,
            bounds,
            len: 0,
            max_per_leaf,
            max_depth,
        }
    }

    /// Number of items in the octree.
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the octree is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Insert a point with an associated index.
    pub fn insert(&mut self, point: Vec3, index: usize) {
        if !self.bounds.contains(point) {
            return;
        }
        Self::insert_recursive(
            &mut self.root,
            point,
            index,
            &self.bounds,
            self.max_per_leaf,
            self.max_depth,
            0,
        );
        self.len += 1;
    }

    fn insert_recursive(
        node: &mut OctNode,
        point: Vec3,
        index: usize,
        bounds: &Aabb,
        max_per_leaf: usize,
        max_depth: usize,
        depth: usize,
    ) {
        match node {
            OctNode::Empty => {
                *node = OctNode::Leaf(vec![(point, index)]);
            }
            OctNode::Leaf(items) => {
                items.push((point, index));
                if items.len() > max_per_leaf && depth < max_depth {
                    let center = bounds.center();
                    let mut children = Box::new([
                        OctNode::Empty,
                        OctNode::Empty,
                        OctNode::Empty,
                        OctNode::Empty,
                        OctNode::Empty,
                        OctNode::Empty,
                        OctNode::Empty,
                        OctNode::Empty,
                    ]);
                    for &(p, idx) in items.iter() {
                        let octant = Self::octant(p, center);
                        let child_bounds = Self::child_bounds(bounds, octant);
                        Self::insert_recursive(
                            &mut children[octant],
                            p,
                            idx,
                            &child_bounds,
                            max_per_leaf,
                            max_depth,
                            depth + 1,
                        );
                    }
                    *node = OctNode::Split {
                        children,
                        bounds: *bounds,
                    };
                }
            }
            OctNode::Split {
                children,
                bounds: node_bounds,
            } => {
                let center = node_bounds.center();
                let octant = Self::octant(point, center);
                let child_bounds = Self::child_bounds(node_bounds, octant);
                Self::insert_recursive(
                    &mut children[octant],
                    point,
                    index,
                    &child_bounds,
                    max_per_leaf,
                    max_depth,
                    depth + 1,
                );
            }
        }
    }

    fn octant(point: Vec3, center: Vec3) -> usize {
        let x = if point.x >= center.x { 1 } else { 0 };
        let y = if point.y >= center.y { 2 } else { 0 };
        let z = if point.z >= center.z { 4 } else { 0 };
        x | y | z
    }

    fn child_bounds(bounds: &Aabb, octant: usize) -> Aabb {
        let center = bounds.center();
        let min = Vec3::new(
            if octant & 1 != 0 {
                center.x
            } else {
                bounds.min.x
            },
            if octant & 2 != 0 {
                center.y
            } else {
                bounds.min.y
            },
            if octant & 4 != 0 {
                center.z
            } else {
                bounds.min.z
            },
        );
        let max = Vec3::new(
            if octant & 1 != 0 {
                bounds.max.x
            } else {
                center.x
            },
            if octant & 2 != 0 {
                bounds.max.y
            } else {
                center.y
            },
            if octant & 4 != 0 {
                bounds.max.z
            } else {
                center.z
            },
        );
        Aabb::new(min, max)
    }

    /// Query all indices of points within the given AABB.
    #[must_use]
    pub fn query_aabb(&self, query: &Aabb) -> Vec<usize> {
        let mut results = Vec::new();
        Self::query_recursive(&self.root, &self.bounds, query, &mut results);
        results
    }

    fn query_recursive(node: &OctNode, bounds: &Aabb, query: &Aabb, results: &mut Vec<usize>) {
        if !aabb_aabb(bounds, query) {
            return;
        }
        match node {
            OctNode::Empty => {}
            OctNode::Leaf(items) => {
                for &(p, idx) in items {
                    if query.contains(p) {
                        results.push(idx);
                    }
                }
            }
            OctNode::Split {
                children,
                bounds: node_bounds,
            } => {
                for oct in 0..8 {
                    let child_bounds = Self::child_bounds(node_bounds, oct);
                    Self::query_recursive(&children[oct], &child_bounds, query, results);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Spatial hash grid
// ---------------------------------------------------------------------------

/// A spatial hash grid for fast broadphase queries.
///
/// Divides space into a uniform grid of cells. Each cell stores indices
/// of items whose positions fall within it. O(1) insert and query for
/// localized spatial lookups.
#[derive(Debug, Clone)]
pub struct SpatialHash {
    inv_cell_size: f32,
    cells: std::collections::HashMap<(i32, i32, i32), Vec<usize>>,
    len: usize,
}

impl SpatialHash {
    /// Create a new spatial hash grid with the given cell size.
    ///
    /// # Errors
    ///
    /// Returns [`crate::HisabError::InvalidInput`] if `cell_size` is not positive.
    pub fn new(cell_size: f32) -> Result<Self, crate::HisabError> {
        if cell_size <= 0.0 {
            return Err(crate::HisabError::InvalidInput(
                "cell_size must be positive".into(),
            ));
        }
        Ok(Self {
            inv_cell_size: 1.0 / cell_size,
            cells: std::collections::HashMap::new(),
            len: 0,
        })
    }

    /// Number of items inserted.
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the grid is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Number of occupied cells.
    #[must_use]
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    fn cell_key(&self, point: Vec3) -> (i32, i32, i32) {
        (
            (point.x * self.inv_cell_size).floor() as i32,
            (point.y * self.inv_cell_size).floor() as i32,
            (point.z * self.inv_cell_size).floor() as i32,
        )
    }

    /// Insert a point with an associated index.
    pub fn insert(&mut self, point: Vec3, index: usize) {
        let key = self.cell_key(point);
        self.cells.entry(key).or_default().push(index);
        self.len += 1;
    }

    /// Clear all items.
    pub fn clear(&mut self) {
        self.cells.clear();
        self.len = 0;
    }

    /// Query all indices in the same cell as the given point.
    #[must_use]
    pub fn query_cell(&self, point: Vec3) -> &[usize] {
        let key = self.cell_key(point);
        self.cells.get(&key).map_or(&[], |v| v.as_slice())
    }

    /// Query all indices within a radius of the given point (broadphase).
    ///
    /// Checks all cells that could contain points within `radius`.
    /// Returns candidate indices — the caller should perform exact distance
    /// checks. Cost is O((2*ceil(r/cell_size)+1)^3) in cell lookups.
    #[must_use]
    pub fn query_radius(&self, point: Vec3, radius: f32) -> Vec<usize> {
        let mut results = Vec::new();
        let cells_r = (radius * self.inv_cell_size).ceil() as i32;
        let center = self.cell_key(point);

        for dx in -cells_r..=cells_r {
            for dy in -cells_r..=cells_r {
                for dz in -cells_r..=cells_r {
                    let key = (center.0 + dx, center.1 + dy, center.2 + dz);
                    if let Some(items) = self.cells.get(&key) {
                        results.extend_from_slice(items);
                    }
                }
            }
        }
        results
    }
}

// ---------------------------------------------------------------------------

// Sort-and-Sweep broadphase (SAP)
// ---------------------------------------------------------------------------

/// Sort-and-Sweep (Sweep and Prune) broadphase collision detection.
///
/// Given a list of AABBs, returns all overlapping pairs.
/// Sweeps along the X axis. O(n log n + k) where k is the number of pairs.
#[must_use]
pub fn sweep_and_prune(aabbs: &[Aabb]) -> Vec<(usize, usize)> {
    let n = aabbs.len();
    if n < 2 {
        return Vec::new();
    }

    // Sort by min.x
    let mut sorted: Vec<usize> = (0..n).collect();
    sorted.sort_unstable_by(|&a, &b| {
        aabbs[a]
            .min
            .x
            .partial_cmp(&aabbs[b].min.x)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut pairs = Vec::new();
    for i in 0..n {
        let a_idx = sorted[i];
        let a_max_x = aabbs[a_idx].max.x;
        for &b_idx in &sorted[(i + 1)..] {
            if aabbs[b_idx].min.x > a_max_x {
                break; // No more overlaps possible on X
            }
            if aabb_aabb(&aabbs[a_idx], &aabbs[b_idx]) {
                let (lo, hi) = if a_idx < b_idx {
                    (a_idx, b_idx)
                } else {
                    (b_idx, a_idx)
                };
                pairs.push((lo, hi));
            }
        }
    }
    pairs
}

// ---------------------------------------------------------------------------
