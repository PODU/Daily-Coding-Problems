// Cousins: BFS level by level; on the target's level collect nodes whose parent differs. O(n) time, O(n) space.
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
        Queue<Node[]> q = new LinkedList<>(); // {node, parent}
        q.add(new Node[]{root, null});
        while (!q.isEmpty()) {
            int sz = q.size();
            Node targetParent = null;
            List<Node[]> level = new ArrayList<>();
            for (int i = 0; i < sz; i++) {
                Node[] cur = q.poll();
                Node node = cur[0], par = cur[1];
                level.add(cur);
                if (node.val == target) targetParent = par;
                if (node.left != null) q.add(new Node[]{node.left, node});
                if (node.right != null) q.add(new Node[]{node.right, node});
            }
            if (targetParent != null) {
                for (Node[] cur : level)
                    if (cur[1] != targetParent && cur[0].val != target)
                        res.add(cur[0].val);
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

        System.out.println("Cousins of 4: " + cousins(root, 4)); // [6]
        System.out.println("Cousins of 6: " + cousins(root, 6)); // [4, 5]
    }
}
