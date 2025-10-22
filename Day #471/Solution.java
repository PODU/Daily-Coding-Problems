// Generate all structurally distinct BSTs with values 1..N via recursive root selection.
// Time: O(Catalan(N) * N), Space: O(Catalan(N) * N) for the trees.
import java.util.ArrayList;
import java.util.List;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static List<Node> generate(int start, int end) {
        List<Node> trees = new ArrayList<>();
        if (start > end) {
            trees.add(null);
            return trees;
        }
        for (int i = start; i <= end; i++) {
            List<Node> lefts = generate(start, i - 1);
            List<Node> rights = generate(i + 1, end);
            for (Node l : lefts) {
                for (Node r : rights) {
                    Node root = new Node(i);
                    root.left = l;
                    root.right = r;
                    trees.add(root);
                }
            }
        }
        return trees;
    }

    static void preorder(Node root, List<Integer> out) {
        if (root == null) return;
        out.add(root.val);
        preorder(root.left, out);
        preorder(root.right, out);
    }

    public static void main(String[] args) {
        int N = 3;
        List<Node> trees = generate(1, N);
        System.out.println("Number of BSTs: " + trees.size());
        for (Node t : trees) {
            List<Integer> out = new ArrayList<>();
            preorder(t, out);
            StringBuilder sb = new StringBuilder();
            for (int i = 0; i < out.size(); i++) {
                if (i > 0) sb.append(" ");
                sb.append(out.get(i));
            }
            System.out.println(sb.toString());
        }
    }
}
