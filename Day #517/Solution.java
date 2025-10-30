// Intersection: two pointers switch lists after end; meet at the join. O(M+N) time, O(1) space.
public class Solution {
    static class Node {
        int val; Node next;
        Node(int v) { val = v; }
    }

    static Node getIntersection(Node a, Node b) {
        if (a == null || b == null) return null;
        Node p = a, q = b;
        while (p != q) {
            p = (p == null) ? b : p.next;
            q = (q == null) ? a : q.next;
        }
        return p;
    }

    public static void main(String[] args) {
        Node shared = new Node(8);
        shared.next = new Node(10);
        Node A = new Node(3); A.next = new Node(7); A.next.next = shared;
        Node B = new Node(99); B.next = new Node(1); B.next.next = shared;
        Node r = getIntersection(A, B);
        System.out.println("The node with value " + r.val);
    }
}
