// Day 751: In-order traversal with O(1) extra space via Morris Traversal.
// Time: O(n), Space: O(1) (re-uses null right pointers as temporary threads).
import java.util.*;

public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static List<Integer> morrisInorder(Node root) {
        List<Integer> out = new ArrayList<>();
        Node cur = root;
        while (cur != null) {
            if (cur.left == null) {
                out.add(cur.val);
                cur = cur.right;
            } else {
                Node pre = cur.left;
                while (pre.right != null && pre.right != cur) pre = pre.right;
                if (pre.right == null) {     // create thread
                    pre.right = cur;
                    cur = cur.left;
                } else {                     // remove thread and visit
                    pre.right = null;
                    out.add(cur.val);
                    cur = cur.right;
                }
            }
        }
        return out;
    }

    public static void main(String[] args) {
        Node root = new Node(4);
        root.left = new Node(2); root.right = new Node(6);
        root.left.left = new Node(1); root.left.right = new Node(3);
        root.right.left = new Node(5); root.right.right = new Node(7);

        List<Integer> res = morrisInorder(root);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < res.size(); i++) {
            sb.append(res.get(i));
            if (i + 1 < res.size()) sb.append(' ');
        }
        System.out.println(sb.toString());
    }
}
