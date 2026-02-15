// K-ary tree symmetry: recursively mirror-match children list. O(n) time/space.
import java.util.*;

public class Solution {
    static class Node {
        int val;
        List<Node> children;
        Node(int v) { val = v; children = new ArrayList<>(); }
    }

    static boolean isMirror(Node L, Node R) {
        if (L == null && R == null) return true;
        if (L == null || R == null || L.val != R.val) return false;
        int n = L.children.size();
        if (R.children.size() != n) return false;
        for (int i = 0; i < n; i++)
            if (!isMirror(L.children.get(i), R.children.get(n - 1 - i))) return false;
        return true;
    }

    static boolean isSymmetric(Node root) {
        if (root == null) return true;
        int n = root.children.size();
        for (int i = 0; i < n / 2; i++)
            if (!isMirror(root.children.get(i), root.children.get(n - 1 - i))) return false;
        return true;
    }

    public static void main(String[] args) {
        // Symmetric: root=4, children=[3,5,3], first 3 has child [9], last 3 has child [9]
        Node root = new Node(4);
        Node c1 = new Node(3); c1.children.add(new Node(9));
        Node c2 = new Node(5);
        Node c3 = new Node(3); c3.children.add(new Node(9));
        root.children.addAll(Arrays.asList(c1, c2, c3));
        System.out.println("Symmetric: " + isSymmetric(root));

        // Asymmetric: root=1, children=[2,3]
        Node r2 = new Node(1);
        r2.children.add(new Node(2));
        r2.children.add(new Node(3));
        System.out.println("Symmetric: " + isSymmetric(r2));
    }
}
