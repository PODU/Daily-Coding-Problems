// Deep clone linked list with random pointers using O(1) interleaving (3 passes).
// Time O(n), Space O(1) extra (besides identity set for verification).
import java.util.IdentityHashMap;
import java.util.Set;
import java.util.Collections;

public class Solution {
    static class Node {
        int val;
        Node next;
        Node random;
        Node(int v) { val = v; }
    }

    static Node cloneList(Node head) {
        if (head == null) return null;
        for (Node cur = head; cur != null; cur = cur.next.next) {
            Node copy = new Node(cur.val);
            copy.next = cur.next;
            cur.next = copy;
        }
        for (Node cur = head; cur != null; cur = cur.next.next) {
            cur.next.random = (cur.random != null) ? cur.random.next : null;
        }
        Node newHead = head.next;
        for (Node cur = head; cur != null; cur = cur.next) {
            Node copy = cur.next;
            cur.next = copy.next;
            copy.next = (copy.next != null) ? copy.next.next : null;
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

        Node cloned = cloneList(n1);

        Set<Node> origSet = Collections.newSetFromMap(new IdentityHashMap<>());
        for (Node cur = n1; cur != null; cur = cur.next) origSet.add(cur);

        boolean deep = true;
        for (Node cur = cloned; cur != null; cur = cur.next) {
            System.out.println("node " + cur.val + " random "
                    + (cur.random != null ? cur.random.val : 0));
            if (origSet.contains(cur)) deep = false;
            if (cur.random != null && origSet.contains(cur.random)) deep = false;
        }
        System.out.println("deep copy verified: " + deep);
    }
}
