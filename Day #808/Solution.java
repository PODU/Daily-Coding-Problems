// Day 808: In-order traversal of a binary tree using O(1) extra space (Morris).
// Thread predecessor's right pointer to current, then unthread. Time O(N), Space O(1).
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
                Node pred = cur.left;
                while (pred.right != null && pred.right != cur) pred = pred.right;
                if (pred.right == null) {
                    pred.right = cur;
                    cur = cur.left;
                } else {
                    pred.right = null;
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
        System.out.println(morrisInorder(root)); // [1, 2, 3, 4, 5, 6, 7]
    }
}
