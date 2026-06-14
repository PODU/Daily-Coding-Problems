// Fisher-Yates shuffle of linked-list node values via array: O(n) time, O(n) space.
// Space-over-time tradeoff: O(1)-extra approach repeatedly picks a random node by traversal in O(n^2) time.
import java.util.*;
public class Solution {
    static class Node { int val; Node next; Node(int v){val=v;} }
    public static void main(String[] args){
        Node head=null, tail=null;
        for(int v=1; v<=5; v++){ Node n=new Node(v); if(head==null){head=tail=n;} else {tail.next=n; tail=n;} }
        List<Node> a=new ArrayList<>();
        for(Node p=head;p!=null;p=p.next) a.add(p);
        int n=a.size();
        Random rng=new Random(12345);
        for(int i=n-1;i>0;i--){ int j=rng.nextInt(i+1); Node t=a.get(i); a.set(i,a.get(j)); a.set(j,t); }
        for(int i=0;i<n;i++) a.get(i).next=(i+1<n?a.get(i+1):null);
        head=a.get(0);
        int[] orig={1,2,3,4,5};
        List<Integer> shuf=new ArrayList<>();
        for(Node p=head;p!=null;p=p.next) shuf.add(p.val);
        Collections.sort(shuf);
        boolean valid=true;
        for(int i=0;i<orig.length;i++) if(orig[i]!=shuf.get(i)) valid=false;
        StringBuilder sb=new StringBuilder();
        for(int i=0;i<orig.length;i++){ if(i>0) sb.append(' '); sb.append(orig[i]); }
        sb.append(" -> ").append(valid?"valid shuffle (same elements)":"INVALID");
        System.out.println(sb.toString());
    }
}
