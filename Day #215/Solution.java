// Day 215: Bottom view of a binary tree.
// Approach: BFS tracking horizontal distance; overwrite map[hd] so last (deepest) node wins. Time O(n log n), Space O(n).
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static List<Integer> bottomView(Node root) {
        TreeMap<Integer, Integer> hdMap = new TreeMap<>();
        if (root == null) return new ArrayList<>();
        Queue<Map.Entry<Node, Integer>> q = new LinkedList<>();
        q.add(new AbstractMap.SimpleEntry<>(root, 0));
        while (!q.isEmpty()) {
            Map.Entry<Node, Integer> e = q.poll();
            Node node = e.getKey();
            int hd = e.getValue();
            hdMap.put(hd, node.val);
            if (node.left != null) q.add(new AbstractMap.SimpleEntry<>(node.left, hd - 1));
            if (node.right != null) q.add(new AbstractMap.SimpleEntry<>(node.right, hd + 1));
        }
        return new ArrayList<>(hdMap.values());
    }

    public static void main(String[] args) {
        Node root = new Node(5);
        root.left = new Node(3); root.right = new Node(7);
        root.left.left = new Node(1); root.left.right = new Node(4);
        root.right.left = new Node(6); root.right.right = new Node(9);
        root.left.left.left = new Node(0);
        root.right.right.left = new Node(8);
        System.out.println(bottomView(root));
    }
}
