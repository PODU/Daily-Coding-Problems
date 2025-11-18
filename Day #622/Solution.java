// Deepest node in a binary tree via BFS level order; last visited node is deepest.
// Time O(N), Space O(N).
import java.util.*;

public class Solution {
    static class Node {
        char val;
        Node left, right;
        Node(char v) { val = v; }
    }

    static char deepest(Node root) {
        if (root == null) return '\0';
        Queue<Node> q = new LinkedList<>();
        q.add(root);
        Node last = root;
        while (!q.isEmpty()) {
            last = q.poll();
            if (last.left != null) q.add(last.left);
            if (last.right != null) q.add(last.right);
        }
        return last.val;
    }

    public static void main(String[] args) {
        Node a = new Node('a');
        Node b = new Node('b');
        Node c = new Node('c');
        Node d = new Node('d');
        a.left = b; a.right = c;
        b.left = d;
        System.out.println(deepest(a)); // d
    }
}
