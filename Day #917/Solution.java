// Partition linked list: stable split into <k and >=k lists, then concatenate.
// Time O(n), Space O(1).
public class Solution {
    static class Node {
        int val; Node next;
        Node(int v) { val = v; }
    }

    static Node partition(Node head, int k) {
        Node lessDummy = new Node(0), geDummy = new Node(0);
        Node lt = lessDummy, ge = geDummy;
        for (Node cur = head; cur != null; cur = cur.next) {
            if (cur.val < k) { lt.next = cur; lt = cur; }
            else { ge.next = cur; ge = cur; }
        }
        ge.next = null;
        lt.next = geDummy.next;
        return lessDummy.next;
    }

    public static void main(String[] args) {
        int[] vals = {5,1,8,0,3};
        Node head = null, tail = null;
        for (int v : vals) {
            Node n = new Node(v);
            if (head == null) { head = tail = n; } else { tail.next = n; tail = n; }
        }
        head = partition(head, 3);
        StringBuilder sb = new StringBuilder();
        for (Node cur = head; cur != null; cur = cur.next) {
            sb.append(cur.val);
            if (cur.next != null) sb.append(" -> ");
        }
        System.out.println(sb.toString());
    }
}
