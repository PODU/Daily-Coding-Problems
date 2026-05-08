// Day 1487: In-order traversal in O(1) space via Morris traversal.
// Time: O(n). Space: O(1) extra (output list aside).
import java.util.ArrayList;
import java.util.List;

public class Solution {
    static class Node {
        int val;
        Node left, right;
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
                Node pred = cur.left;
                while (pred.right != null && pred.right != cur) pred = pred.right;
                if (pred.right == null) {
                    pred.right = cur;       // thread
                    cur = cur.left;
                } else {
                    pred.right = null;      // restore
                    out.add(cur.val);
                    cur = cur.right;
                }
            }
        }
        return out;
    }

    public static void main(String[] args) {
        Node root = new Node(4);
        root.left = new Node(2);
        root.right = new Node(6);
        root.left.left = new Node(1);
        root.left.right = new Node(3);
        root.right.left = new Node(5);
        root.right.right = new Node(7);

        List<Integer> res = morrisInorder(root);
        StringBuilder sb = new StringBuilder("In-order: ");
        for (int i = 0; i < res.size(); i++) {
            sb.append(res.get(i));
            if (i + 1 < res.size()) sb.append(' ');
        }
        System.out.println(sb.toString());
    }
}
