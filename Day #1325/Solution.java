// Day 1325: Sorted array -> height-balanced BST.
// Recursively pick the middle element as the root so both halves differ in height by <=1. O(n) time, O(log n) stack.
import java.util.*;

public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static Node build(int[] a, int lo, int hi) {
        if (lo > hi) return null;
        int mid = lo + (hi - lo) / 2;
        Node root = new Node(a[mid]);
        root.left = build(a, lo, mid - 1);
        root.right = build(a, mid + 1, hi);
        return root;
    }

    static void preorder(Node n, List<Integer> out) {
        if (n == null) return;
        out.add(n.val);
        preorder(n.left, out);
        preorder(n.right, out);
    }

    public static void main(String[] args) {
        int[] a = {1, 2, 3, 4, 5, 6, 7};
        Node root = build(a, 0, a.length - 1);
        List<Integer> out = new ArrayList<>();
        preorder(root, out);
        System.out.println("preorder: " + out); // preorder: [4, 2, 1, 3, 6, 5, 7]
    }
}
