// Rotate singly linked list right by k. Make ring, break at n-(k%n). O(n) time, O(1) space.
public class Solution {
    static class Node { int v; Node next; Node(int x){v=x;} }

    static Node rotateRight(Node head, int k) {
        if (head == null || head.next == null) return head;
        int n = 1; Node tail = head;
        while (tail.next != null) { tail = tail.next; n++; }
        tail.next = head;                 // ring
        int steps = n - (k % n);
        Node newTail = head;
        for (int i = 1; i < steps; i++) newTail = newTail.next;
        Node newHead = newTail.next;
        newTail.next = null;
        return newHead;
    }

    static Node build(int... xs) {
        Node h=null,t=null;
        for (int x: xs){ Node nd=new Node(x); if(h==null){h=t=nd;} else {t.next=nd;t=nd;} }
        return h;
    }
    static void print(Node h){
        StringBuilder sb=new StringBuilder();
        for(Node p=h;p!=null;p=p.next){ sb.append(p.v); if(p.next!=null) sb.append(" -> "); }
        System.out.println(sb.toString());
    }

    public static void main(String[] a){
        print(rotateRight(build(7,7,3,5), 2));
        print(rotateRight(build(1,2,3,4,5), 3));
    }
}
