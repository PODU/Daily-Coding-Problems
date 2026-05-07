// Day 1485: Determine whether a k-ary tree is symmetric about its root.
// Subtrees mirror iff values match and child i mirrors child (k-1-i).
// Time O(N), Space O(H).
import java.util.*;

public class Solution {
    static class Node {
        int val;
        List<Node> children;
        Node(int v, Node... c) { val = v; children = Arrays.asList(c); }
    }

    static boolean isMirror(Node a, Node b) {
        if (a.val != b.val || a.children.size() != b.children.size()) return false;
        int k = a.children.size();
        for (int i = 0; i < k; ++i)
            if (!isMirror(a.children.get(i), b.children.get(k - 1 - i))) return false;
        return true;
    }

    static boolean isSymmetric(Node root) {
        return root == null || isMirror(root, root);
    }

    public static void main(String[] args) {
        Node tree = new Node(4,
            new Node(3, new Node(9)),
            new Node(5),
            new Node(3, new Node(9)));
        System.out.println(isSymmetric(tree));  // true
    }
}
