// Deep clone list w/ random ptr: interleave clones, wire randoms, unweave. O(n) time, O(1) extra.
public class Solution {
    static class Node {
        int val;
        Node next, random;
        Node(int v) { val = v; }
    }

    static Node copyRandomList(Node head) {
        if (head == null) return null;
        for (Node c = head; c != null; c = c.next.next) {
            Node cl = new Node(c.val);
            cl.next = c.next;
            c.next = cl;
        }
        for (Node c = head; c != null; c = c.next.next)
            c.next.random = (c.random != null) ? c.random.next : null;
        Node newHead = head.next;
        for (Node c = head; c != null; c = c.next) {
            Node cl = c.next;
            c.next = cl.next;
            cl.next = (cl.next != null) ? cl.next.next : null;
        }
        return newHead;
    }

    public static void main(String[] args) {
        Node n1 = new Node(1), n2 = new Node(2), n3 = new Node(3), n4 = new Node(4);
        n1.next = n2; n2.next = n3; n3.next = n4;
        n1.random = n3;
        n2.random = n1;
        n3.random = n3;
        n4.random = n2;

        Node cloned = copyRandomList(n1);
        for (Node c = cloned; c != null; c = c.next)
            System.out.println("node " + c.val + ", random " + c.random.val);
    }
}
