// Day 963: Find intersecting node of two singly linked lists.
// Approach: two pointers swap heads; meet at intersection. Time O(M+N), Space O(1).
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
        Node n8 = new Node(8);
        n8.next = new Node(10);
        Node a = new Node(3); a.next = new Node(7); a.next.next = n8;
        Node b = new Node(99); b.next = new Node(1); b.next.next = n8;

        Node res = getIntersection(a, b);
        System.out.println("the node with value " + res.val);
    }
}
