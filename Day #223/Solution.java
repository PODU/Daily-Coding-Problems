// Day 223: In-order traversal with O(1) extra space (Morris traversal).
// Approach: thread each node to its in-order predecessor temporarily, remove thread after visiting.
// Time O(n), Space O(1) (no stack/recursion).
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static List<Integer> morrisInorder(Node root) {
        List<Integer> res = new ArrayList<>();
        Node cur = root;
        while (cur != null) {
            if (cur.left == null) {
                res.add(cur.val);
                cur = cur.right;
            } else {
                Node pred = cur.left;
                while (pred.right != null && pred.right != cur) pred = pred.right;
                if (pred.right == null) {
                    pred.right = cur;     // create thread
                    cur = cur.left;
                } else {
                    pred.right = null;    // remove thread
                    res.add(cur.val);
                    cur = cur.right;
                }
            }
        }
        return res;
    }

    public static void main(String[] args) {
        Node root = new Node(4);
        root.left = new Node(2); root.right = new Node(6);
        root.left.left = new Node(1); root.left.right = new Node(3);
        root.right.left = new Node(5); root.right.right = new Node(7);
        System.out.println(morrisInorder(root)); // [1, 2, 3, 4, 5, 6, 7]
    }
}
