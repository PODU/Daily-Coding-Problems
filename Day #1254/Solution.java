// Interval point stabbing: greedy, sort by right endpoint, pick endpoint when uncovered.
// Time O(n log n), Space O(n).
import java.util.*;
public class Solution {
    public static void main(String[] args){
        int[][] iv={{1,4},{4,5},{7,9},{9,12}};
        Arrays.sort(iv,(a,b)->Integer.compare(a[1],b[1]));
        List<Integer> pts=new ArrayList<>(); long last=Long.MIN_VALUE;
        for(int[] p:iv){ if(p[0]>last){ last=p[1]; pts.add(p[1]); } }
        StringBuilder sb=new StringBuilder("[");
        for(int i=0;i<pts.size();++i){ sb.append(pts.get(i)); if(i+1<pts.size())sb.append(", "); }
        sb.append("]");
        System.out.println(sb);
    }
}
