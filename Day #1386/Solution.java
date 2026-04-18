// Second largest in BST via reverse in-order (right,node,left), stop at 2nd visited node. O(h) space, O(n) worst time.
import java.util.*;

public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static int secondLargest(Node root) {
        Deque<Node> st = new ArrayDeque<>();
        Node cur = root;
        int count = 0;
        while (cur != null || !st.isEmpty()) {
            while (cur != null) { st.push(cur); cur = cur.right; }
            cur = st.pop();
            if (++count == 2) return cur.val;
            cur = cur.left;
        }
        return -1;
    }

    public static void main(String[] args) {
        Node root = new Node(5);
        root.left = new Node(3);
        root.left.left = new Node(2);
        root.left.right = new Node(4);
        root.right = new Node(8);
        root.right.left = new Node(7);
        root.right.right = new Node(9);
        System.out.println(secondLargest(root));
    }
}
