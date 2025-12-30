// Merge two binary trees recursively; node value = sum, missing nodes taken from the other.
// Time: O(n), Space: O(h) recursion.
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static Node merge(Node a, Node b) {
        if (a == null) return b;
        if (b == null) return a;
        Node n = new Node(a.val + b.val);
        n.left = merge(a.left, b.left);
        n.right = merge(a.right, b.right);
        return n;
    }

    static void preorder(Node n, List<Integer> out) {
        if (n == null) return;
        out.add(n.val);
        preorder(n.left, out);
        preorder(n.right, out);
    }

    public static void main(String[] args) {
        Node t1 = new Node(1);
        t1.left = new Node(3);
        t1.left.left = new Node(5);
        t1.right = new Node(2);

        Node t2 = new Node(2);
        t2.left = new Node(1);
        t2.left.right = new Node(4);
        t2.right = new Node(3);
        t2.right.right = new Node(7);

        Node m = merge(t1, t2);
        List<Integer> out = new ArrayList<>();
        preorder(m, out);
        System.out.println(out.toString());
    }
}
