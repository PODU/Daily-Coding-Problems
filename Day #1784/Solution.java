// Morris in-order traversal: thread tree via predecessor links for O(1) space.
// Time O(N), Space O(1) (excluding output).
public class Solution {
    static class Node { int val; Node left, right; Node(int v){val=v;} }

    static void morrisInorder(Node root) {
        StringBuilder sb = new StringBuilder();
        Node cur = root;
        while (cur != null) {
            if (cur.left == null) {
                if (sb.length() > 0) sb.append(' ');
                sb.append(cur.val);
                cur = cur.right;
            } else {
                Node pre = cur.left;
                while (pre.right != null && pre.right != cur) pre = pre.right;
                if (pre.right == null) { pre.right = cur; cur = cur.left; }
                else {
                    pre.right = null;
                    if (sb.length() > 0) sb.append(' ');
                    sb.append(cur.val);
                    cur = cur.right;
                }
            }
        }
        System.out.println(sb.toString());
    }

    public static void main(String[] args) {
        Node root = new Node(4);
        root.left = new Node(2);
        root.right = new Node(5);
        root.left.left = new Node(1);
        root.left.right = new Node(3);
        morrisInorder(root);
    }
}
