// Sort a linked list in O(n log n) time, O(1) extra space.
// Bottom-up (iterative) merge sort on the list; no recursion stack -> constant space.
public class Solution {
    static class Node {
        int val;
        Node next;
        Node(int v) { val = v; }
    }

    static Node split(Node head, int size) {
        for (int i = 1; head != null && i < size; i++) head = head.next;
        if (head == null) return null;
        Node rest = head.next;
        head.next = null;
        return rest;
    }

    static Node merge(Node a, Node b, Node tail) {
        while (a != null && b != null) {
            if (a.val <= b.val) { tail.next = a; a = a.next; }
            else { tail.next = b; b = b.next; }
            tail = tail.next;
        }
        tail.next = (a != null) ? a : b;
        while (tail.next != null) tail = tail.next;
        return tail;
    }

    static Node sortList(Node head) {
        if (head == null || head.next == null) return head;
        int n = 0;
        for (Node p = head; p != null; p = p.next) n++;
        Node dummy = new Node(0);
        dummy.next = head;
        for (int size = 1; size < n; size *= 2) {
            Node cur = dummy.next;
            Node tail = dummy;
            while (cur != null) {
                Node left = cur;
                Node right = split(left, size);
                cur = split(right, size);
                tail = merge(left, right, tail);
            }
        }
        return dummy.next;
    }

    public static void main(String[] args) {
        int[] vals = {4, 1, -3, 99};
        Node dummy = new Node(0);
        Node t = dummy;
        for (int v : vals) { t.next = new Node(v); t = t.next; }
        Node head = sortList(dummy.next);
        StringBuilder sb = new StringBuilder();
        for (Node p = head; p != null; p = p.next) {
            if (sb.length() > 0) sb.append(" -> ");
            sb.append(p.val);
        }
        System.out.println(sb.toString());
    }
}
