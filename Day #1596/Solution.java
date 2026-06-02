// Approach: Symmetric k-ary tree via isMirror recursion comparing children mirror-wise.
// Time O(n), Space O(h) recursion.
import java.util.*;

public class Solution {
    static class Node {
        int val;
        List<Node> children = new ArrayList<>();
        Node(int v) { val = v; }
    }

    static boolean isMirror(Node a, Node b) {
        if (a == null && b == null) return true;
        if (a == null || b == null) return false;
        if (a.val != b.val) return false;
        if (a.children.size() != b.children.size()) return false;
        int k = a.children.size();
        for (int i = 0; i < k; i++)
            if (!isMirror(a.children.get(i), b.children.get(k - 1 - i))) return false;
        return true;
    }

    static boolean isSymmetric(Node root) {
        if (root == null) return true;
        return isMirror(root, root);
    }

    public static void main(String[] args) {
        Node root = new Node(4);
        Node c1 = new Node(3); c1.children.add(new Node(9));
        Node c2 = new Node(5);
        Node c3 = new Node(3); c3.children.add(new Node(9));
        root.children.add(c1); root.children.add(c2); root.children.add(c3);
        System.out.println(isSymmetric(root) ? "true" : "false");
    }
}
