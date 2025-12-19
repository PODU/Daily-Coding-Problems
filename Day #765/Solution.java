// Day 765: Remove the kth-from-last node in one pass with two pointers.
// fast leads slow by k; when fast hits the end, slow precedes the target.
// Time: O(n), Space: O(1).
public class Solution {
    static class Node {
        int val; Node next;
        Node(int v) { val = v; }
    }

    static Node removeKthLast(Node head, int k) {
        Node dummy = new Node(0);
        dummy.next = head;
        Node fast = dummy, slow = dummy;
        for (int i = 0; i < k; i++) fast = fast.next;
        while (fast.next != null) { fast = fast.next; slow = slow.next; }
        slow.next = slow.next.next;   // unlink target
        return dummy.next;
    }

    static void printList(Node head) {
        StringBuilder sb = new StringBuilder();
        for (Node p = head; p != null; p = p.next) {
            sb.append(p.val);
            if (p.next != null) sb.append(" -> ");
        }
        System.out.println(sb);
    }

    public static void main(String[] args) {
        Node head = new Node(1);
        Node cur = head;
        for (int v = 2; v <= 5; v++) { cur.next = new Node(v); cur = cur.next; }

        System.out.print("before: "); printList(head);
        head = removeKthLast(head, 2);
        System.out.print("after:  "); printList(head);   // 1 -> 2 -> 3 -> 5
    }
}
