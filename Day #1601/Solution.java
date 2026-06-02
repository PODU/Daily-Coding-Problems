// Min root-to-leaf path sum via recursive DFS; leaf returns its val, internal node adds min of existing children.
// Reconstruct path by tracking the chosen child. Time O(n), space O(h).
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    // returns min path sum from node to a leaf, fills `path` with node values.
    static int minPath(Node node, List<Integer> path) {
        path.add(node.val);
        if (node.left == null && node.right == null) return node.val;

        int best = Integer.MAX_VALUE;
        List<Integer> bestSub = null;
        for (Node child : new Node[]{node.left, node.right}) {
            if (child == null) continue;
            List<Integer> sub = new ArrayList<>();
            int s = minPath(child, sub);
            if (s < best) { best = s; bestSub = sub; }
        }
        path.addAll(bestSub);
        return node.val + best;
    }

    public static void main(String[] args) {
        Node root = new Node(10);
        root.left = new Node(5);
        root.right = new Node(5);
        root.left.right = new Node(2);
        root.right.right = new Node(1);
        root.right.right.left = new Node(-1);

        List<Integer> path = new ArrayList<>();
        int sum = minPath(root, path);

        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < path.size(); i++) {
            if (i > 0) sb.append(", ");
            sb.append(path.get(i));
        }
        System.out.println("The minimum path is [" + sb + "], which has sum " + sum + ".");
    }
}
