// Day 1151: Palindrome linked list (singly).
// Find middle via slow/fast, reverse 2nd half, compare. O(n) time, O(1) space.
public class Solution {
    static class Node { int val; Node next; Node(int v) { val = v; } }

    static boolean isPalindrome(Node head) {
        if (head == null || head.next == null) return true;
        Node slow = head, fast = head;
        while (fast.next != null && fast.next.next != null) { slow = slow.next; fast = fast.next.next; }
        Node prev = null, cur = slow.next;
        while (cur != null) { Node nx = cur.next; cur.next = prev; prev = cur; cur = nx; }
        Node p1 = head, p2 = prev;
        while (p2 != null) { if (p1.val != p2.val) return false; p1 = p1.next; p2 = p2.next; }
        return true;
    }

    static Node build(int[] v) {
        Node dummy = new Node(0), t = dummy;
        for (int x : v) { t.next = new Node(x); t = t.next; }
        return dummy.next;
    }

    public static void main(String[] args) {
        System.out.println(isPalindrome(build(new int[]{1, 4, 3, 4, 1})) ? "True" : "False"); // True
        System.out.println(isPalindrome(build(new int[]{1, 4})) ? "True" : "False");          // False
    }
}
