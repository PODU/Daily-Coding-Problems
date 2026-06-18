// Sort a singly linked list via bottom-up (iterative) merge sort.
// O(n log n) time, O(1) extra space (no recursion).
public class Solution {
    static class Node { int val; Node next; Node(int v){ val=v; } }

    static int length(Node h){ int n=0; while(h!=null){ n++; h=h.next; } return n; }

    // Cut list to first n nodes; return head of the remainder.
    static Node split(Node head, int n){
        for(int i=1; head!=null && i<n; i++) head=head.next;
        if(head==null) return null;
        Node second=head.next; head.next=null; return second;
    }

    static Node merge(Node a, Node b){
        Node dummy=new Node(0), tail=dummy;
        while(a!=null && b!=null){
            if(a.val<=b.val){ tail.next=a; a=a.next; }
            else { tail.next=b; b=b.next; }
            tail=tail.next;
        }
        tail.next = (a!=null) ? a : b;
        return dummy.next;
    }

    static Node sortList(Node head){
        if(head==null) return null;
        int n=length(head);
        Node dummy=new Node(0); dummy.next=head;
        for(int size=1; size<n; size<<=1){
            Node prev=dummy, cur=dummy.next;
            while(cur!=null){
                Node left=cur;
                Node right=split(left,size);
                cur=split(right,size);
                prev.next=merge(left,right);
                while(prev.next!=null) prev=prev.next;
            }
        }
        return dummy.next;
    }

    public static void main(String[] args){
        int[] vals={4,1,-3,99};
        Node head=null, tail=null;
        for(int v:vals){ Node nd=new Node(v); if(head==null){head=tail=nd;} else {tail.next=nd;tail=nd;} }
        head=sortList(head);
        StringBuilder sb=new StringBuilder();
        for(Node p=head;p!=null;p=p.next){ if(sb.length()>0) sb.append(" -> "); sb.append(p.val); }
        System.out.println(sb.toString());
    }
}
