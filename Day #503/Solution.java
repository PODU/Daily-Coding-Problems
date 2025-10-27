// Day 503: Sort a singly linked list using bottom-up (iterative) merge sort.
// Time O(n log n), Space O(1) auxiliary (no recursion).
public class Solution {
    static class Node {
        int val;
        Node next;
        Node(int v) { val = v; }
    }

    static int listLength(Node head) {
        int n = 0;
        for (; head != null; head = head.next) n++;
        return n;
    }

    // Split off `size` nodes starting at head; cut there and return the rest.
    static Node split(Node head, int size) {
        for (int i = 1; head != null && i < size; i++) head = head.next;
        if (head == null) return null;
        Node rest = head.next;
        head.next = null;
        return rest;
    }

    // Merge two sorted lists after `tail`, return new tail.
    static Node mergeLists(Node a, Node b, Node tail) {
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
        int n = listLength(head);
        Node dummy = new Node(0);
        dummy.next = head;
        for (int size = 1; size < n; size *= 2) {
            Node tail = dummy;
            Node cur = dummy.next;
            while (cur != null) {
                Node left = cur;
                Node right = split(left, size);
                cur = split(right, size);
                tail = mergeLists(left, right, tail);
            }
        }
        return dummy.next;
    }

    static void printList(Node head) {
        StringBuilder sb = new StringBuilder();
        boolean first = true;
        for (; head != null; head = head.next) {
            if (!first) sb.append(" -> ");
            sb.append(head.val);
            first = false;
        }
        System.out.println(sb.toString());
    }

    public static void main(String[] args) {
        int[] vals = {4, 1, -3, 99};
        Node dummy = new Node(0);
        Node tail = dummy;
        for (int v : vals) { tail.next = new Node(v); tail = tail.next; }
        Node sorted = sortList(dummy.next);
        printList(sorted); // -3 -> 1 -> 4 -> 99
    }
}
