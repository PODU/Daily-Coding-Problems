// Min broadcast range: sort towers, binary-search nearest per listener, answer = max of mins.
// Time O((N+M) log M), Space O(1).
import java.util.*;
public class Solution {
    static int minRange(int[] listeners,int[] towers){
        Arrays.sort(towers);
        int ans=0;
        for(int l:listeners){
            int idx=Arrays.binarySearch(towers,l);
            if(idx<0)idx=-idx-1;
            long best=Long.MAX_VALUE;
            if(idx<towers.length)best=Math.min(best,(long)towers[idx]-l);
            if(idx>0)best=Math.min(best,(long)l-towers[idx-1]);
            ans=Math.max(ans,(int)best);
        }
        return ans;
    }
    public static void main(String[] args){
        System.out.println(minRange(new int[]{1,5,11,20},new int[]{4,8,15}));
    }
}
