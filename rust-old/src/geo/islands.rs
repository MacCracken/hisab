//! Island detection for rigid-body simulation.
//!
//! Builds a contact graph from body-pair edges and partitions bodies into
//! connected components (islands) using union-find with path compression and
//! union-by-rank. Islands that contain only sleeping bodies are returned
//! separately so the caller can skip integrating them.

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// A contact island — a connected component in the contact graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Island {
    /// Body indices belonging to this island.
    pub bodies: Vec<usize>,
    /// Indices into the original `contacts` slice whose both endpoints live
    /// in this island.
    pub contacts: Vec<usize>,
}

/// Contact edge: a contact between two bodies in the simulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContactEdge {
    /// Index of the first body.
    pub body_a: usize,
    /// Index of the second body.
    pub body_b: usize,
}

// ---------------------------------------------------------------------------
// Union-find (path-compressed, union-by-rank)
// ---------------------------------------------------------------------------

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    /// Find the representative root of `x`, with path compression.
    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            // Path halving — compress two levels at a time.
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    /// Union the sets containing `a` and `b`.  Returns `false` if they were
    /// already in the same set.
    fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return false;
        }
        // Union by rank: attach the smaller tree under the larger.
        match self.rank[ra].cmp(&self.rank[rb]) {
            std::cmp::Ordering::Less => self.parent[ra] = rb,
            std::cmp::Ordering::Greater => self.parent[rb] = ra,
            std::cmp::Ordering::Equal => {
                self.parent[rb] = ra;
                self.rank[ra] += 1;
            }
        }
        true
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Detect islands (connected components) in the contact graph.
///
/// Uses union-find for O(n·α(n)) performance.
/// Bodies with no contacts form singleton islands.
///
/// # Arguments
/// - `num_bodies`: total number of bodies in the simulation.
/// - `contacts`: contact edges (pairs of body indices).  Any edge whose
///   `body_a` or `body_b` is `>= num_bodies` is silently ignored.
/// - `sleeping`: optional slice of per-body sleep flags.  When provided,
///   islands where **every** body is sleeping are returned in the second
///   element of the returned tuple.  The slice may be shorter than
///   `num_bodies`; missing entries are treated as awake.
///
/// # Returns
/// `(active_islands, sleeping_islands)` — two `Vec<Island>` values.
/// When `sleeping` is `None`, the second vec is always empty.
///
/// # Panics
/// Never panics.  Out-of-range body indices in `contacts` are skipped.
#[must_use]
pub fn detect_islands(
    num_bodies: usize,
    contacts: &[ContactEdge],
    sleeping: Option<&[bool]>,
) -> (Vec<Island>, Vec<Island>) {
    if num_bodies == 0 {
        return (Vec::new(), Vec::new());
    }

    let mut uf = UnionFind::new(num_bodies);

    // --- Build the forest from valid contact edges -------------------------
    for edge in contacts {
        if edge.body_a < num_bodies && edge.body_b < num_bodies {
            uf.union(edge.body_a, edge.body_b);
        }
    }

    // --- Collect components: root → (bodies list, contacts list) ----------
    //
    // Use a flat Vec arena keyed by root index to avoid HashMap overhead.
    // `component_index[root]` gives the index in `components`, or usize::MAX
    // if this root hasn't been seen yet.
    let mut component_index: Vec<usize> = vec![usize::MAX; num_bodies];
    let mut bodies_per_component: Vec<Vec<usize>> = Vec::new();

    for body in 0..num_bodies {
        let root = uf.find(body);
        if component_index[root] == usize::MAX {
            component_index[root] = bodies_per_component.len();
            bodies_per_component.push(Vec::new());
        }
        let idx = component_index[root];
        bodies_per_component[idx].push(body);
    }

    let num_components = bodies_per_component.len();
    let mut contacts_per_component: Vec<Vec<usize>> = vec![Vec::new(); num_components];

    for (ci, edge) in contacts.iter().enumerate() {
        if edge.body_a < num_bodies && edge.body_b < num_bodies {
            let root = uf.find(edge.body_a);
            let idx = component_index[root];
            contacts_per_component[idx].push(ci);
        }
    }

    // --- Classify active vs. sleeping -------------------------------------
    let is_body_sleeping = |body: usize| -> bool {
        match sleeping {
            Some(s) => *s.get(body).unwrap_or(&false),
            None => false,
        }
    };

    let mut active_islands: Vec<Island> = Vec::new();
    let mut sleeping_islands: Vec<Island> = Vec::new();

    for (idx, bodies) in bodies_per_component.into_iter().enumerate() {
        let island = Island {
            bodies,
            contacts: contacts_per_component[idx].clone(),
        };
        if sleeping.is_some() && island.bodies.iter().all(|&b| is_body_sleeping(b)) {
            sleeping_islands.push(island);
        } else {
            active_islands.push(island);
        }
    }

    (active_islands, sleeping_islands)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: collect all body sets from a list of islands, sorted for stable
    // comparison.
    fn sorted_body_sets(islands: &[Island]) -> Vec<Vec<usize>> {
        let mut sets: Vec<Vec<usize>> = islands
            .iter()
            .map(|i| {
                let mut b = i.bodies.clone();
                b.sort_unstable();
                b
            })
            .collect();
        sets.sort();
        sets
    }

    #[test]
    fn no_bodies_returns_empty() {
        let (active, sleeping) = detect_islands(0, &[], None);
        assert!(active.is_empty());
        assert!(sleeping.is_empty());
    }

    #[test]
    fn singleton_islands_no_contacts() {
        // 4 bodies, no contacts → 4 singleton islands.
        let (active, sleeping) = detect_islands(4, &[], None);
        assert_eq!(active.len(), 4);
        assert!(sleeping.is_empty());
        let sets = sorted_body_sets(&active);
        assert_eq!(sets, vec![vec![0], vec![1], vec![2], vec![3]]);
    }

    #[test]
    fn single_contact_merges_two_bodies() {
        let contacts = [ContactEdge {
            body_a: 0,
            body_b: 1,
        }];
        let (active, _) = detect_islands(3, &contacts, None);
        let sets = sorted_body_sets(&active);
        // Bodies {0,1} merged; body 2 remains singleton.
        assert!(sets.contains(&vec![0, 1]));
        assert!(sets.contains(&vec![2]));
        assert_eq!(active.len(), 2);
    }

    #[test]
    fn chain_merges_all_into_one_island() {
        // 0-1, 1-2, 2-3 → single island {0,1,2,3}
        let contacts = [
            ContactEdge {
                body_a: 0,
                body_b: 1,
            },
            ContactEdge {
                body_a: 1,
                body_b: 2,
            },
            ContactEdge {
                body_a: 2,
                body_b: 3,
            },
        ];
        let (active, _) = detect_islands(4, &contacts, None);
        assert_eq!(active.len(), 1);
        let mut bodies = active[0].bodies.clone();
        bodies.sort_unstable();
        assert_eq!(bodies, vec![0, 1, 2, 3]);
    }

    #[test]
    fn contacts_attributed_to_correct_island() {
        // Island A: bodies 0,1 via contact index 0.
        // Island B: bodies 2,3 via contact index 1.
        let contacts = [
            ContactEdge {
                body_a: 0,
                body_b: 1,
            },
            ContactEdge {
                body_a: 2,
                body_b: 3,
            },
        ];
        let (active, _) = detect_islands(4, &contacts, None);
        assert_eq!(active.len(), 2);
        for island in &active {
            let mut b = island.bodies.clone();
            b.sort_unstable();
            if b == vec![0, 1] {
                assert_eq!(island.contacts, vec![0]);
            } else if b == vec![2, 3] {
                assert_eq!(island.contacts, vec![1]);
            } else {
                panic!("unexpected island bodies: {b:?}");
            }
        }
    }

    #[test]
    fn out_of_range_contacts_ignored() {
        // body_b = 99 is out of range for num_bodies = 3 → no merge.
        let contacts = [ContactEdge {
            body_a: 0,
            body_b: 99,
        }];
        let (active, _) = detect_islands(3, &contacts, None);
        // All three should be singletons.
        assert_eq!(active.len(), 3);
    }

    #[test]
    fn duplicate_contacts_do_not_split_island() {
        // Same pair appearing twice should still be one island.
        let contacts = [
            ContactEdge {
                body_a: 0,
                body_b: 1,
            },
            ContactEdge {
                body_a: 0,
                body_b: 1,
            },
        ];
        let (active, _) = detect_islands(2, &contacts, None);
        assert_eq!(active.len(), 1);
        // Both contact indices should be listed.
        let mut c = active[0].contacts.clone();
        c.sort_unstable();
        assert_eq!(c, vec![0, 1]);
    }

    #[test]
    fn sleeping_bodies_form_sleeping_island() {
        // Bodies 0 and 1 in contact, both sleeping → sleeping island.
        // Body 2 awake → active singleton.
        let contacts = [ContactEdge {
            body_a: 0,
            body_b: 1,
        }];
        let sleeping = [true, true, false];
        let (active, sleeping_islands) = detect_islands(3, &contacts, Some(&sleeping));
        // One sleeping island {0,1}, one active island {2}.
        assert_eq!(sleeping_islands.len(), 1);
        let mut sb = sleeping_islands[0].bodies.clone();
        sb.sort_unstable();
        assert_eq!(sb, vec![0, 1]);
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].bodies, vec![2]);
    }

    #[test]
    fn mixed_island_is_active_not_sleeping() {
        // Bodies 0 (sleeping) and 1 (awake) in contact → active island.
        let contacts = [ContactEdge {
            body_a: 0,
            body_b: 1,
        }];
        let sleeping = [true, false];
        let (active, sleeping_islands) = detect_islands(2, &contacts, Some(&sleeping));
        assert_eq!(sleeping_islands.len(), 0);
        assert_eq!(active.len(), 1);
    }

    #[test]
    fn no_sleeping_slice_means_no_sleeping_islands() {
        let contacts = [ContactEdge {
            body_a: 0,
            body_b: 1,
        }];
        let (active, sleeping_islands) = detect_islands(2, &contacts, None);
        assert!(sleeping_islands.is_empty());
        assert_eq!(active.len(), 1);
    }

    #[test]
    fn sleeping_slice_shorter_than_num_bodies_treated_as_awake() {
        // sleeping has 1 entry (body 0 = true). body 1 has no entry → awake.
        // They share a contact → mixed → active.
        let contacts = [ContactEdge {
            body_a: 0,
            body_b: 1,
        }];
        let sleeping = [true]; // body 1 implicitly awake
        let (active, sleeping_islands) = detect_islands(2, &contacts, Some(&sleeping));
        assert!(sleeping_islands.is_empty());
        assert_eq!(active.len(), 1);
    }

    #[test]
    fn large_disconnected_graph() {
        // 10 bodies, 3 separate pairs and 4 singletons.
        let contacts = [
            ContactEdge {
                body_a: 0,
                body_b: 1,
            },
            ContactEdge {
                body_a: 2,
                body_b: 3,
            },
            ContactEdge {
                body_a: 4,
                body_b: 5,
            },
        ];
        let (active, _) = detect_islands(10, &contacts, None);
        // 3 pairs + 4 singletons = 7 islands.
        assert_eq!(active.len(), 7);
    }

    #[test]
    fn star_topology_single_island() {
        // Body 0 touches bodies 1,2,3,4 → all in one island.
        let contacts = [
            ContactEdge {
                body_a: 0,
                body_b: 1,
            },
            ContactEdge {
                body_a: 0,
                body_b: 2,
            },
            ContactEdge {
                body_a: 0,
                body_b: 3,
            },
            ContactEdge {
                body_a: 0,
                body_b: 4,
            },
        ];
        let (active, _) = detect_islands(5, &contacts, None);
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].bodies.len(), 5);
    }
}
