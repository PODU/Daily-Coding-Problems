// Lazy binary tree: generate() returns a root in O(1) whose children are thunks
// (Supplier) forced on demand; a seeded coin flip (<1 continue prob) makes the tree
// finite w.p.1 but unbounded. Realization is depth-capped for a deterministic demo.
import java.util.function.Supplier;

public class Solution {
    static class LCG {
        long s;
        LCG(long seed) { s = seed; }
        long next() { s = (s * 16807L) % 2147483647L; return s; } // Park-Miller
        boolean coin() { return next() % 100 < 45; } // child exists prob 0.45 -> finite
    }

    static class Node {
        int val;
        Supplier<Node> left, right;
    }

    static int[] counter = {0};

    // makeNode/generate do NOT force children: O(1).
    static Node makeNode(LCG rng) {
        Node node = new Node();
        node.val = counter[0]++;
        node.left = () -> rng.coin() ? makeNode(rng) : null;
        node.right = () -> rng.coin() ? makeNode(rng) : null;
        return node;
    }

    static Node generate(LCG rng) {
        return makeNode(rng); // O(1): one node, children unevaluated
    }

    static int realize(Node node, int depth, int cap) {
        int count = 1;
        if (depth < cap) {
            Node l = node.left.get();
            if (l != null) count += realize(l, depth + 1, cap);
            Node r = node.right.get();
            if (r != null) count += realize(r, depth + 1, cap);
        }
        return count;
    }

    public static void main(String[] args) {
        LCG rng = new LCG(42);
        Node root = generate(rng); // returns instantly
        System.out.println("generate() returned a lazy tree root in O(1)");
        int n = realize(root, 0, 6);
        System.out.println("Realized tree node count: " + n);
    }
}
