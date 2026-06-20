// Two-pointer intersection of singly linked lists: redirect each pointer to the other head at end.
// Time O(M+N), Space O(1).
public class Solution {
    static class Node {
        int val;
        Node next;
        Node(int v) { val = v; }
    }

    static Node getIntersection(Node headA, Node headB) {
        if (headA == null || headB == null) return null;
        Node pA = headA, pB = headB;
        while (pA != pB) {
            pA = (pA == null) ? headB : pA.next;
            pB = (pB == null) ? headA : pB.next;
        }
        return pA;
    }

    public static void main(String[] args) {
        Node n8 = new Node(8);
        n8.next = new Node(10);
        Node a = new Node(3);
        a.next = new Node(7);
        a.next.next = n8;
        Node b = new Node(99);
        b.next = new Node(1);
        b.next.next = n8;

        Node res = getIntersection(a, b);
        System.out.println(res != null ? res.val : "null");
    }
}
