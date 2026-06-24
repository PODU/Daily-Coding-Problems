// Cousins: BFS tracking parent & depth; find target's depth+parent, collect nodes at
// same depth with different parent. Time O(n), Space O(n).
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static List<Integer> cousins(Node root, int target) {
        Queue<Node[]> q = new LinkedList<>(); // {node, parent}
        q.add(new Node[]{root, null});
        while (!q.isEmpty()) {
            int sz = q.size();
            List<Node[]> level = new ArrayList<>();
            Node targetParent = null;
            boolean found = false;
            for (int i = 0; i < sz; i++) {
                Node[] cur = q.poll();
                Node node = cur[0], par = cur[1];
                level.add(cur);
                if (node.val == target) { targetParent = par; found = true; }
                if (node.left != null) q.add(new Node[]{node.left, node});
                if (node.right != null) q.add(new Node[]{node.right, node});
            }
            if (found) {
                List<Integer> res = new ArrayList<>();
                for (Node[] cur : level)
                    if (cur[1] != targetParent) res.add(cur[0].val);
                return res;
            }
        }
        return new ArrayList<>();
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
