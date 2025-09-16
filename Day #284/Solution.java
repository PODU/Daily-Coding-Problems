// Day 284: Find all cousins of a node (same level, different parent) via BFS.
// Time O(N), Space O(N).
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static List<Integer> cousins(Node root, int target) {
        Queue<Node> q = new LinkedList<>();
        q.add(root);
        while (!q.isEmpty()) {
            int sz = q.size();
            List<Integer> level = new ArrayList<>();
            Node targetParent = null;
            for (int i = 0; i < sz; i++) {
                Node n = q.poll();
                for (Node c : new Node[]{n.left, n.right}) {
                    if (c != null) {
                        level.add(c.val);
                        if (c.val == target) targetParent = n;
                        q.add(c);
                    }
                }
            }
            if (targetParent != null) {
                Set<Integer> sib = new HashSet<>();
                if (targetParent.left != null) sib.add(targetParent.left.val);
                if (targetParent.right != null) sib.add(targetParent.right.val);
                List<Integer> res = new ArrayList<>();
                for (int v : level) if (!sib.contains(v)) res.add(v);
                return res;
            }
        }
        return new ArrayList<>();
    }

    public static void main(String[] args) {
        Node root = new Node(1);
        root.left = new Node(2); root.right = new Node(3);
        root.left.left = new Node(4); root.left.right = new Node(5);
        root.right.right = new Node(6);
        System.out.println(cousins(root, 4)); // [6]
    }
}
