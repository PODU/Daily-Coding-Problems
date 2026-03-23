// Max contiguous subarray sum, empty allowed: Kadane, clamp running sum at 0.
// Time O(n), Space O(1).
public class Solution {
    static long maxSub(int[] a){
        long cur=0,best=0;
        for(int x:a){ cur+=x; if(cur<0)cur=0; best=Math.max(best,cur); }
        return best;
    }
    public static void main(String[] args){
        System.out.println(maxSub(new int[]{34,-50,42,14,-5,86}));
        System.out.println(maxSub(new int[]{-5,-1,-8,-9}));
    }
}
