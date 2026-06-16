// Day 1677: Linked-list palindrome. Singly: find middle, reverse 2nd half, compare.
// Doubly: two pointers from both ends. Time O(n), Space O(1).
public class Solution {
    static class Node {
        int val; Node next, prev;
        Node(int v) { val = v; }
    }

    static Node build(int[] v) {
        Node head = null, tail = null;
        for (int x : v) {
            Node n = new Node(x);
            if (head == null) head = tail = n;
            else { tail.next = n; n.prev = tail; tail = n; }
        }
        return head;
    }

    static boolean isPalindrome(Node head) {
        Node slow = head, fast = head;
        while (fast != null && fast.next != null) { slow = slow.next; fast = fast.next.next; }
        Node prev = null;
        while (slow != null) { Node nx = slow.next; slow.next = prev; prev = slow; slow = nx; }
        Node a = head, b = prev;
        while (b != null) { if (a.val != b.val) return false; a = a.next; b = b.next; }
        return true;
    }

    public static void main(String[] args) {
        System.out.println(isPalindrome(build(new int[]{1,4,3,4,1})) ? "True" : "False"); // True
        System.out.println(isPalindrome(build(new int[]{1,4})) ? "True" : "False");       // False
    }
}
