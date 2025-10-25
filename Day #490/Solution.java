// Day 490: Bottom view of a binary tree.
// BFS by horizontal distance (root 0, left hd-1, right hd+1); the last node seen in BFS
// order for each hd is the lowest. Output sorted by hd. Time O(n log n), Space O(n).
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static List<Integer> bottomView(Node root) {
        List<Integer> res = new ArrayList<>();
        if (root == null) return res;
        TreeMap<Integer, Integer> hdToVal = new TreeMap<>();
        Queue<Object[]> q = new LinkedList<>(); // {node, hd}
        q.add(new Object[]{root, 0});
        while (!q.isEmpty()) {
            Object[] cur = q.poll();
            Node node = (Node) cur[0];
            int hd = (Integer) cur[1];
            hdToVal.put(hd, node.val);
            if (node.left != null) q.add(new Object[]{node.left, hd - 1});
            if (node.right != null) q.add(new Object[]{node.right, hd + 1});
        }
        res.addAll(hdToVal.values());
        return res;
    }

    public static void main(String[] args) {
        Node root = new Node(5);
        root.left = new Node(3);
        root.right = new Node(7);
        root.left.left = new Node(1);
        root.left.right = new Node(4);
        root.right.left = new Node(6);
        root.right.right = new Node(9);
        root.left.left.left = new Node(0);
        root.right.right.left = new Node(8);
        System.out.println(bottomView(root)); // [0, 1, 3, 6, 8, 9]
    }
}
