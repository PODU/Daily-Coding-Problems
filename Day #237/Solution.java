// Symmetric k-ary tree: a tree is symmetric iff left subtree mirrors right subtree.
// Recursively compare children in mirrored order. Time: O(N), Space: O(height).
import java.util.*;

public class Solution {
    static class Node {
        int val;
        List<Node> children = new ArrayList<>();
        Node(int v) { val = v; }
    }

    static boolean mirror(Node a, Node b) {
        if (a == null && b == null) return true;
        if (a == null || b == null) return false;
        if (a.val != b.val) return false;
        if (a.children.size() != b.children.size()) return false;
        int n = a.children.size();
        for (int i = 0; i < n; i++)
            if (!mirror(a.children.get(i), b.children.get(n - 1 - i))) return false;
        return true;
    }

    static boolean isSymmetric(Node root) {
        if (root == null) return true;
        int n = root.children.size();
        for (int i = 0; i < n / 2; i++)
            if (!mirror(root.children.get(i), root.children.get(n - 1 - i))) return false;
        return true;
    }

    public static void main(String[] args) {
        Node root = new Node(4);
        Node a = new Node(3); a.children.add(new Node(9));
        Node b = new Node(5);
        Node c = new Node(3); c.children.add(new Node(9));
        root.children = Arrays.asList(a, b, c);
        System.out.println(isSymmetric(root)); // true
    }
}
