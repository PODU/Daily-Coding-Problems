// Day 863: generate() returns a finite but arbitrarily large binary tree in O(1).
// Approach: root created in O(1); children expanded lazily with randomized termination
// (each child exists with prob < 0.5 => branching process is finite almost surely).
// generate(): O(1). Materializing whole tree: O(size). Deterministic demo via MINSTD RNG.
public class Solution {
    static long rngState = 42;
    static double nextRand() {
        rngState = (rngState * 48271) % 2147483647;
        return (double) rngState / 2147483647.0;
    }

    static final double P = 0.45;
    static final int DEPTH_CAP = 50;

    static class Node {
        Node left, right;
        boolean expanded = false;
    }

    static void ensureChildren(Node n, int depth) {
        if (n.expanded) return;
        n.expanded = true;
        if (depth >= DEPTH_CAP) return;
        if (nextRand() < P) n.left = new Node();
        if (nextRand() < P) n.right = new Node();
    }

    static Node generate() { return new Node(); }  // O(1)

    static int countNodes(Node n, int depth) {
        if (n == null) return 0;
        ensureChildren(n, depth);
        return 1 + countNodes(n.left, depth + 1) + countNodes(n.right, depth + 1);
    }

    public static void main(String[] args) {
        Node root = generate();
        System.out.println("Generated a finite binary tree with " + countNodes(root, 0)
                + " nodes (deterministic demo).");
    }
}
