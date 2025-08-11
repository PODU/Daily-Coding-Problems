// Day 104: Linked-list palindrome (singly or doubly). Find middle, reverse second
// half, compare both halves. O(n) time, O(1) extra space.
public class Solution {
    static class Node {
        int val; Node next;
        Node(int v) { val = v; }
    }

    static boolean isPalindrome(Node head) {
        Node slow = head, fast = head;
        while (fast != null && fast.next != null) { slow = slow.next; fast = fast.next.next; }
        Node prev = null;
        while (slow != null) { Node nx = slow.next; slow.next = prev; prev = slow; slow = nx; }
        Node l = head, r = prev;
        while (r != null) { if (l.val != r.val) return false; l = l.next; r = r.next; }
        return true;
    }

    static Node build(int[] vals) {
        Node dummy = new Node(0), cur = dummy;
        for (int v : vals) { cur.next = new Node(v); cur = cur.next; }
        return dummy.next;
    }

    public static void main(String[] args) {
        System.out.println(isPalindrome(build(new int[]{1, 4, 3, 4, 1}))); // true
        System.out.println(isPalindrome(build(new int[]{1, 4})));          // false
    }
}
