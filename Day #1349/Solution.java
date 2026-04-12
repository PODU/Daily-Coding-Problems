// Day 1349: Reconstruct a BST from its postorder traversal.
// Consume postorder right-to-left with value bounds (right subtree before left). O(n) time, O(h) space.
import java.util.*;

public class Solution {
    static class Node { int val; Node left, right; Node(int v){ val = v; } }

    static int idx;
    static int[] post;

    static Node build(long bound) {
        if (idx < 0 || post[idx] < bound) return null;
        Node node = new Node(post[idx--]);
        node.right = build(node.val);
        node.left = build(bound);
        return node;
    }

    static void preorder(Node n, List<Integer> out){ if(n==null)return; out.add(n.val); preorder(n.left,out); preorder(n.right,out);}
    static void inorder(Node n, List<Integer> out){ if(n==null)return; inorder(n.left,out); out.add(n.val); inorder(n.right,out);}

    public static void main(String[] args) {
        post = new int[]{2, 4, 3, 8, 7, 5};
        idx = post.length - 1;
        Node root = build(Long.MIN_VALUE);
        List<Integer> pre = new ArrayList<>(), in = new ArrayList<>();
        preorder(root, pre); inorder(root, in);
        System.out.println("Root: " + root.val);
        StringBuilder sb = new StringBuilder("Preorder: ");
        for (int x : pre) sb.append(x).append(" ");
        System.out.println(sb.toString().trim());
        sb = new StringBuilder("Inorder: ");
        for (int x : in) sb.append(x).append(" ");
        System.out.println(sb.toString().trim());
    }
}
