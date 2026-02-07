// Sorted array -> height-balanced BST: recursively pick middle as root.
// Time: O(n), Space: O(log n) recursion.
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
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

    static void printRotated(Node node, int depth) {
        if (node == null) return;
        printRotated(node.right, depth + 1);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < depth * 4; i++) sb.append(' ');
        System.out.println(sb.toString() + node.val);
        printRotated(node.left, depth + 1);
    }

    static void inorder(Node node, List<Integer> out) {
        if (node == null) return;
        inorder(node.left, out);
        out.add(node.val);
        inorder(node.right, out);
    }

    public static void main(String[] args) {
        int[] a = {-10, -3, 0, 5, 9};
        Node root = build(a, 0, a.length - 1);
        System.out.println("Height-balanced BST (rotated 90 deg):");
        printRotated(root, 0);
        List<Integer> io = new ArrayList<>();
        inorder(root, io);
        StringBuilder sb = new StringBuilder("In-order:");
        for (int x : io) sb.append(" ").append(x);
        System.out.println(sb.toString());
    }
}
