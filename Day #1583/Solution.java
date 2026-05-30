// Day 1583: Bottom view of a binary tree.
// BFS tracking horizontal distance; last node seen per hd (deepest wins). Time: O(n log n); Space: O(n).
import java.util.*;

public class Solution {
    static class Node { int val; Node l, r; Node(int v){ val = v; } }

    static List<Integer> bottomView(Node root) {
        TreeMap<Integer, Integer> hdVal = new TreeMap<>();
        if (root == null) return new ArrayList<>();
        Queue<Map.Entry<Node, Integer>> q = new LinkedList<>();
        q.add(new AbstractMap.SimpleEntry<>(root, 0));
        while (!q.isEmpty()) {
            Map.Entry<Node, Integer> e = q.poll();
            Node n = e.getKey(); int hd = e.getValue();
            hdVal.put(hd, n.val);
            if (n.l != null) q.add(new AbstractMap.SimpleEntry<>(n.l, hd - 1));
            if (n.r != null) q.add(new AbstractMap.SimpleEntry<>(n.r, hd + 1));
        }
        return new ArrayList<>(hdVal.values());
    }

    public static void main(String[] args) {
        Node root = new Node(5);
        root.l = new Node(3); root.r = new Node(7);
        root.l.l = new Node(1); root.l.r = new Node(4);
        root.r.l = new Node(6); root.r.r = new Node(9);
        root.l.l.l = new Node(0);
        root.r.r.l = new Node(8);
        System.out.println(bottomView(root)); // [0, 1, 3, 6, 8, 9]
    }
}
