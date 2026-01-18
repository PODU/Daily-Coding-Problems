// Complete-tree node count: if left height == right height subtree is perfect (2^h-1), else recurse. O(log^2 n).
public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static int leftHeight(Node n)  { int h = 0; while (n != null) { h++; n = n.left;  } return h; }
    static int rightHeight(Node n) { int h = 0; while (n != null) { h++; n = n.right; } return h; }

    static int countNodes(Node root) {
        if (root == null) return 0;
        int lh = leftHeight(root), rh = rightHeight(root);
        if (lh == rh) return (1 << lh) - 1; // perfect subtree
        return 1 + countNodes(root.left) + countNodes(root.right);
    }

    public static void main(String[] args) {
        // Complete tree with 6 nodes (values 1..6 in level order):
        //         1
        //       /   \
        //      2     3
        //     / \   /
        //    4   5 6
        Node[] n = new Node[7];
        for (int v = 1; v <= 6; v++) n[v] = new Node(v);
        n[1].left = n[2]; n[1].right = n[3];
        n[2].left = n[4]; n[2].right = n[5];
        n[3].left = n[6];

        System.out.println(countNodes(n[1])); // 6
    }
}
