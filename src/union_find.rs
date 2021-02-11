use cargo_snippet::snippet;

#[snippet("@union_find")]
pub struct UnionFind {
    parents: Vec<Option<usize>>,
    sizes: Vec<usize>,
    groups_count: usize,
}

#[snippet("@union_find")]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parents: vec![None; n],
            sizes: vec![1; n],
            groups_count: n,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if let Some(parent) = self.parents[x] {
            let root_x = self.find(parent);
            self.parents[x] = Some(root_x);
            root_x
        } else {
            x
        }
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let (root_x, root_y) = (self.find(x), self.find(y));

        if root_x == root_y {
            return false;
        }

        if self.sizes[root_x] >= self.sizes[root_y] {
            self.parents[root_y] = Some(root_x);
            self.sizes[root_x] += self.sizes[root_y];
        } else {
            self.parents[root_x] = Some(root_y);
            self.sizes[root_y] += self.sizes[root_x];
        }
        self.groups_count -= 1;
        true
    }

    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn groups_count(&self) -> usize {
        self.groups_count
    }

    pub fn size(&mut self, x: usize) -> usize {
        let root_x = self.find(x);
        self.sizes[root_x]
    }
}

#[test]
fn union_find() {
    let mut uf = UnionFind::new(5);
    assert_eq!(uf.groups_count(), 5);

    uf.union(0, 1);
    assert_eq!(uf.groups_count(), 4);

    uf.union(2, 3);
    assert_eq!(uf.groups_count(), 3);

    uf.union(0, 4);
    assert_eq!(uf.groups_count(), 2);

    // is_same
    assert!(uf.is_same(0, 1));
    assert!(uf.is_same(2, 3));
    assert!(uf.is_same(0, 4));
    assert!(uf.is_same(1, 4));
    assert!(!uf.is_same(0, 2));
    assert!(!uf.is_same(3, 4));

    // size
    assert_eq!(uf.size(0), 3);
    assert_eq!(uf.size(1), 3);
    assert_eq!(uf.size(2), 2);
    assert_eq!(uf.size(3), 2);
    assert_eq!(uf.size(4), 3);
}
