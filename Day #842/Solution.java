// Day 842: invert (mirror) a binary tree by swapping children at every node.
// Recursive DFS. O(n) time, O(h) stack space.
import java.util.*;

public class Solution {
    static class Node {
        char v; Node l, r;
        Node(char c){ v = c; }
    }

    static Node invert(Node root){
        if(root == null) return null;
        Node t = root.l; root.l = root.r; root.r = t;
        invert(root.l);
        invert(root.r);
        return root;
    }

    static String levelOrder(Node root){
        StringBuilder sb = new StringBuilder();
        if(root == null) return "";
        Queue<Node> q = new LinkedList<>();
        q.add(root);
        while(!q.isEmpty()){
            Node n = q.poll();
            if(sb.length() > 0) sb.append(' ');
            sb.append(n.v);
            if(n.l != null) q.add(n.l);
            if(n.r != null) q.add(n.r);
        }
        return sb.toString();
    }

    public static void main(String[] args){
        Node a=new Node('a'),b=new Node('b'),c=new Node('c'),
             d=new Node('d'),e=new Node('e'),f=new Node('f');
        a.l=b; a.r=c; b.l=d; b.r=e; c.l=f;
        invert(a);
        System.out.println(levelOrder(a)); // a c b f e d
    }
}
