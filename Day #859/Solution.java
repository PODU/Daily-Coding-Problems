// Day 859: Bottom view of a binary tree.
// Approach: BFS by horizontal distance; last node seen at each hd wins (lowest).
// Time: O(n log n) for ordering, Space: O(n).
import java.util.*;

public class Solution {
    static class Node { int val; Node l, r; Node(int v){ val = v; } }

    static List<Integer> bottomView(Node root) {
        TreeMap<Integer, Integer> map = new TreeMap<>();
        Queue<Node> nodes = new LinkedList<>();
        Queue<Integer> hds = new LinkedList<>();
        nodes.add(root); hds.add(0);
        while (!nodes.isEmpty()) {
            Node n = nodes.poll(); int hd = hds.poll();
            map.put(hd, n.val);
            if (n.l != null) { nodes.add(n.l); hds.add(hd - 1); }
            if (n.r != null) { nodes.add(n.r); hds.add(hd + 1); }
        }
        return new ArrayList<>(map.values());
    }

    public static void main(String[] args) {
        Node root = new Node(5);
        root.l = new Node(3); root.r = new Node(7);
        root.l.l = new Node(1); root.l.r = new Node(4);
        root.r.l = new Node(6); root.r.r = new Node(9);
        root.l.l.l = new Node(0);
        root.r.r.l = new Node(8);
        System.out.println(bottomView(root));
    }
}
