// Prefix-sum hashmap: store sum->index; if sum-K seen, slice between. O(n) time, O(n) space.
import java.util.*;
public class Solution {
    public static void main(String[] args){
        int[] a={1,2,3,4,5};int K=9;
        Map<Long,Integer> seen=new HashMap<>();seen.put(0L,-1);long s=0;int lo=-1,hi=-1;
        for(int i=0;i<a.length;++i){s+=a[i];
            if(seen.containsKey(s-K)){lo=seen.get(s-K)+1;hi=i;break;}
            seen.putIfAbsent(s,i);}
        StringBuilder sb=new StringBuilder("[");
        for(int i=lo;i<=hi;++i){sb.append(a[i]);if(i<hi)sb.append(", ");}
        sb.append("]");System.out.println(sb);
    }
}
