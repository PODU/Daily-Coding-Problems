// LCA via parent pointers: equalize depths then walk up together. O(h) time, O(1) space.
// Tree stored in a Vec arena; nodes referenced by index. Root has parent = usize::MAX.
struct Tree {
    val: Vec<i32>,
    parent: Vec<usize>,
}

const NONE: usize = usize::MAX;

impl Tree {
    fn depth(&self, mut n: usize) -> i32 {
        let mut d = 0;
        while self.parent[n] != NONE {
            n = self.parent[n];
            d += 1;
        }
        d
    }

    fn lca(&self, mut a: usize, mut b: usize) -> usize {
        let (mut da, mut db) = (self.depth(a), self.depth(b));
        while da > db {
            a = self.parent[a];
            da -= 1;
        }
        while db > da {
            b = self.parent[b];
            db -= 1;
        }
        while a != b {
            a = self.parent[a];
            b = self.parent[b];
        }
        a
    }
}

fn main() {
    // indices 0..7 represent node values 0..7; value 0 unused.
    let val = vec![0, 1, 2, 3, 4, 5, 6, 7];
    // parent[i] = parent index of node i
    let mut parent = vec![NONE; 8];
    parent[2] = 1; parent[3] = 1;
    parent[4] = 2; parent[5] = 2;
    parent[6] = 3; parent[7] = 3;
    let tree = Tree { val, parent };

    println!("{}", tree.val[tree.lca(4, 5)]);
    println!("{}", tree.val[tree.lca(4, 6)]);
}
