// Cousins in a binary tree: BFS level by level tracking parent; collect same-level
// nodes with a different parent than target. Time O(n), Space O(n).
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static List<Integer> cousins(Node root, int target) {
        List<Integer> res = new ArrayList<>();
        if (root == null) return res;
        Queue<Node[]> q = new LinkedList<>(); // [node, parent]
        q.add(new Node[]{root, null});
        while (!q.isEmpty()) {
            int sz = q.size();
            List<Node[]> level = new ArrayList<>();
            Node targetParent = null;
            boolean found = false;
            for (int i = 0; i < sz; i++) {
                Node[] cur = q.poll();
                level.add(cur);
                if (cur[0].val == target) {
                    found = true;
                    targetParent = cur[1];
                }
                if (cur[0].left != null) q.add(new Node[]{cur[0].left, cur[0]});
                if (cur[0].right != null) q.add(new Node[]{cur[0].right, cur[0]});
            }
            if (found) {
                for (Node[] p : level)
                    if (p[0].val != target && p[1] != targetParent)
                        res.add(p[0].val);
                return res;
            }
        }
        return res;
    }

    public static void main(String[] args) {
        Node root = new Node(1);
        root.left = new Node(2);
        root.right = new Node(3);
        root.left.left = new Node(4);
        root.left.right = new Node(5);
        root.right.right = new Node(6);

        System.out.println(cousins(root, 4));
    }
}
