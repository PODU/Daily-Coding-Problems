// Day 1412: generate() returns a finite but arbitrarily large binary tree in O(1).
// Approach: lazy nodes — children are materialized only on access (here via a deterministic
// LCG so the demo is reproducible). generate() itself is O(1); a child appears on first touch.
public class Solution {
    static long lcg = 42L;
    static int nextRand() { lcg = lcg * 1103515245L + 12345L; return (int)((lcg >> 16) & 0x7fff); }

    static class LazyNode {
        int depth;
        boolean leftDone = false, rightDone = false;
        LazyNode leftCache = null, rightCache = null;
        LazyNode(int d) { depth = d; }

        boolean spawn() { int bound = 55 - depth * 7; return bound > 0 && (nextRand() % 100) < bound; }
        LazyNode left()  { if (!leftDone)  { leftDone = true;  if (spawn()) leftCache = new LazyNode(depth + 1); } return leftCache; }
        LazyNode right() { if (!rightDone) { rightDone = true; if (spawn()) rightCache = new LazyNode(depth + 1); } return rightCache; }
    }

    // generate(): O(1)
    static LazyNode generate() { return new LazyNode(0); }

    static int countNodes(LazyNode n) {
        if (n == null) return 0;
        int l = countNodes(n.left());
        int r = countNodes(n.right());
        return 1 + l + r;
    }

    public static void main(String[] args) {
        LazyNode root = generate();
        System.out.println("Tree size: " + countNodes(root));
    }
}
