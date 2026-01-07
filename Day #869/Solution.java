// Day 869: Is a linked list a palindrome (works for singly linked; doubly trivial).
// Approach: find middle (slow/fast), reverse second half, compare both halves. O(1) space.
// Time: O(n), Space: O(1).
public class Solution {
    static class Node { int val; Node next; Node(int v){ val = v; } }

    static Node build(int[] vals) {
        Node head = null, tail = null;
        for (int v : vals) {
            Node n = new Node(v);
            if (head == null) head = tail = n;
            else { tail.next = n; tail = n; }
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
