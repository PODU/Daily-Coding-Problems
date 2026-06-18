// BFS level order; last node dequeued is a deepest node. Time O(n), Space O(n).
import java.util.ArrayDeque;
import java.util.Queue;

public class Solution {
    static class Node {
        char val;
        Node left, right;
        Node(char v) { val = v; }
    }

    static Node deepestNode(Node root) {
        if (root == null) return null;
        Queue<Node> q = new ArrayDeque<>();
        q.add(root);
        Node last = root;
        while (!q.isEmpty()) {
            last = q.poll();
            if (last.left != null) q.add(last.left);
            if (last.right != null) q.add(last.right);
        }
        return last;
    }

    public static void main(String[] args) {
        Node a = new Node('a');
        Node b = new Node('b');
        Node c = new Node('c');
        Node d = new Node('d');
        a.left = b; a.right = c;
        b.left = d;
        System.out.println(deepestNode(a).val);
    }
}
