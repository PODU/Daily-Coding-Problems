// Reconstruct BST from postorder: iterate right-to-left as (root,right,left) with a lower bound. O(n) time/space.
import java.util.*;

public class Solution {
    static class Node { int val; Node left, right; Node(int v){ val=v; } }

    static int idx;
    static int[] post;

    static Node build(int lower){
        if(idx < 0 || post[idx] < lower) return null;
        int val = post[idx]; idx--;
        Node node = new Node(val);
        node.right = build(val);
        node.left  = build(lower);
        return node;
    }

    static void inorder(Node n, List<Integer> o){ if(n!=null){ inorder(n.left,o); o.add(n.val); inorder(n.right,o); } }
    static void postorder(Node n, List<Integer> o){ if(n!=null){ postorder(n.left,o); postorder(n.right,o); o.add(n.val); } }

    public static void main(String[] args){
        post = new int[]{2,4,3,8,7,5};
        idx = post.length - 1;
        Node root = build(Integer.MIN_VALUE);
        List<Integer> ino = new ArrayList<>(), po = new ArrayList<>();
        inorder(root, ino); postorder(root, po);
        StringBuilder s1 = new StringBuilder("Inorder:");   for(int x: ino) s1.append(" ").append(x);
        StringBuilder s2 = new StringBuilder("Postorder:"); for(int x: po)  s2.append(" ").append(x);
        System.out.println(s1);
        System.out.println(s2);
    }
}
