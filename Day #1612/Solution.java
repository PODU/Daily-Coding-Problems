// Sorted array -> height-balanced BST: pick lower-middle as root, recurse. Print preorder.
// mid = (lo+hi)/2 (lower-middle). Time O(n), Space O(log n) recursion.
import java.util.ArrayList;
import java.util.List;
import java.util.StringJoiner;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static Node build(int[] a, int lo, int hi) {
        if (lo > hi) return null;
        int mid = (lo + hi) / 2;
        Node root = new Node(a[mid]);
        root.left = build(a, lo, mid - 1);
        root.right = build(a, mid + 1, hi);
        return root;
    }

    static void preorder(Node node, List<Integer> out) {
        if (node == null) return;
        out.add(node.val);
        preorder(node.left, out);
        preorder(node.right, out);
    }

    public static void main(String[] args) {
        int[] a = {-10, -3, 0, 5, 9};
        Node root = build(a, 0, a.length - 1);
        List<Integer> out = new ArrayList<>();
        preorder(root, out);
        StringJoiner sj = new StringJoiner(" ");
        for (int v : out) sj.add(Integer.toString(v));
        System.out.println(sj.toString());
    }
}
