// Convert to full binary tree by removing single-child nodes (post-order recursion).
// O(N) time, O(H) space.
public class Solution {
    static class Node {
        int val; Node l, r;
        Node(int v){ val = v; }
    }
    static Node fullify(Node n){
        if(n == null) return null;
        n.l = fullify(n.l);
        n.r = fullify(n.r);
        if(n.l == null && n.r != null) return n.r;
        if(n.l != null && n.r == null) return n.l;
        return n;
    }
    static String serialize(Node n){
        if(n == null) return "";
        if(n.l == null && n.r == null) return Integer.toString(n.val);
        return n.val + "(" + serialize(n.l) + ", " + serialize(n.r) + ")";
    }
    public static void main(String[] args){
        Node n0=new Node(0), n1=new Node(1), n2=new Node(2), n3=new Node(3);
        Node n4=new Node(4), n5=new Node(5), n6=new Node(6), n7=new Node(7);
        n0.l=n1; n0.r=n2; n1.l=n3; n2.r=n4; n3.r=n5; n4.l=n6; n4.r=n7;
        Node root = fullify(n0);
        System.out.println(serialize(root)); // 0(5, 4(6, 7))
    }
}
