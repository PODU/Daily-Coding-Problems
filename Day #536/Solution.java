// Reconstruct BST from postorder: scan right-to-left (preorder of root,right,left)
// with an upper-bound recursion. Time O(n), space O(n).
import java.util.*;

public class Solution {
    static class Node { int val; Node left, right; Node(int v){ val=v; } }

    static int idx;
    static Node build(int[] post, int bound) {
        if (idx < 0 || post[idx] < bound) return null;
        Node root = new Node(post[idx--]);
        root.right = build(post, root.val);
        root.left  = build(post, bound);
        return root;
    }

    static void preorder(Node n, List<Integer> out){ if(n==null)return; out.add(n.val); preorder(n.left,out); preorder(n.right,out); }
    static void inorder(Node n, List<Integer> out){ if(n==null)return; inorder(n.left,out); out.add(n.val); inorder(n.right,out); }

    public static void main(String[] args) {
        int[] post = {2,4,3,8,7,5};
        idx = post.length - 1;
        Node root = build(post, Integer.MIN_VALUE);
        List<Integer> pre = new ArrayList<>(), in = new ArrayList<>();
        preorder(root, pre); inorder(root, in);
        StringBuilder sb = new StringBuilder("preorder:");
        for (int x: pre) sb.append(' ').append(x);
        sb.append("\ninorder: ");
        for (int x: in) sb.append(' ').append(x);
        System.out.println(sb);
    }
}
