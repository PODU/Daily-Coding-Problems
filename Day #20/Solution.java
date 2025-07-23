// Intersection of two linked lists: two-pointer switch trick.
// Time O(M+N), Space O(1).
public class Solution {
    static class Node {
        int val;
        Node next;
        Node(int v) { val = v; }
    }

    static Node getIntersection(Node a, Node b) {
        Node pa = a, pb = b;
        while (pa != pb) {
            pa = (pa == null) ? b : pa.next;
            pb = (pb == null) ? a : pb.next;
        }
        return pa;
    }

    public static void main(String[] args) {
        Node shared = new Node(8);
        shared.next = new Node(10);

        Node a = new Node(3);
        a.next = new Node(7);
        a.next.next = shared;

        Node b = new Node(99);
        b.next = new Node(1);
        b.next.next = shared;

        Node res = getIntersection(a, b);
        System.out.println(res.val);
    }
}
