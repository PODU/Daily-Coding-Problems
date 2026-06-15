// Two-pointer merge from both ends of sorted array; larger abs square goes to back. O(n) time, O(n) space.
import java.util.*;
public class Solution {
    public static void main(String[] args){
        int[] a={-9,-2,0,2,3};int n=a.length;long[] r=new long[n];int l=0,h=n-1;
        for(int p=n-1;p>=0;--p){long lo=(long)a[l]*a[l],hi=(long)a[h]*a[h];
            if(lo>hi){r[p]=lo;++l;}else{r[p]=hi;--h;}}
        StringBuilder sb=new StringBuilder("[");
        for(int i=0;i<n;++i){sb.append(r[i]);if(i+1<n)sb.append(", ");}
        sb.append("]");System.out.println(sb);
    }
}
