// Day 966: Deep clone a linked list where each node has a random pointer.
// Approach: interleave copies, set randoms, split. Time O(n), Space O(1) extra.
public class Solution {
    static class Node {
        int val; Node next, random;
        Node(int v) { val = v; }
    }

    static Node cloneList(Node head) {
        if (head == null) return null;
        for (Node p = head; p != null; p = p.next.next) {
            Node c = new Node(p.val);
            c.next = p.next;
            p.next = c;
        }
        for (Node p = head; p != null; p = p.next.next)
            if (p.random != null) p.next.random = p.random.next;
        Node newHead = head.next;
        for (Node p = head; p != null; p = p.next) {
            Node c = p.next;
            p.next = c.next;
            if (c.next != null) c.next = c.next.next;
        }
        return newHead;
    }

    public static void main(String[] args) {
        Node a = new Node(1), b = new Node(2), c = new Node(3);
        a.next = b; b.next = c;
        a.random = c; b.random = a; c.random = b;

        Node cl = cloneList(a);
        for (Node p = cl; p != null; p = p.next)
            System.out.println("val=" + p.val + " random=" + (p.random != null ? p.random.val : -1));
    }
}
