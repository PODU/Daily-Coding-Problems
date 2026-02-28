// Bottom view via BFS tracking horizontal distance; last (deepest) node per hd wins. O(n log n).
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
        Node(int v, Node l, Node r) { val = v; left = l; right = r; }
    }

    public static void main(String[] args) {
        Node root = new Node(5,
            new Node(3, new Node(1, new Node(0), null), new Node(4)),
            new Node(7, new Node(6), new Node(9, new Node(8), null)));
        TreeMap<Integer, Integer> map = new TreeMap<>();
        Deque<Object[]> q = new ArrayDeque<>();
        q.add(new Object[]{root, 0});
        while (!q.isEmpty()) {
            Object[] cur = q.poll();
            Node node = (Node) cur[0];
            int hd = (Integer) cur[1];
            map.put(hd, node.val);
            if (node.left != null) q.add(new Object[]{node.left, hd - 1});
            if (node.right != null) q.add(new Object[]{node.right, hd + 1});
        }
        List<String> parts = new ArrayList<>();
        for (int v : map.values()) parts.add(Integer.toString(v));
        System.out.println("[" + String.join(", ", parts) + "]");
    }
}
