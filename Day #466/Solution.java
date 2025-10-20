// Symmetric k-ary tree: recursively compare children of two subtrees in mirror order.
// Time: O(n), Space: O(h) recursion.
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
        return mirror(root, root);
    }

    public static void main(String[] args) {
        Node root = new Node(4);
        Node l3 = new Node(3), m5 = new Node(5), r3 = new Node(3);
        root.children = Arrays.asList(l3, m5, r3);
        l3.children = Arrays.asList(new Node(9));
        r3.children = Arrays.asList(new Node(9));
        System.out.println(isSymmetric(root) ? "True" : "False");
    }
}
