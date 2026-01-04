// Day 852: maximum circular subarray sum (empty allowed -> 0).
// answer = max(maxKadane clamped at 0, total - minKadane). O(n) time, O(1) space.
public class Solution {
    static long maxCircular(long[] a){
        long total = 0, maxK = Long.MIN_VALUE, minK = Long.MAX_VALUE;
        long curMax = 0, curMin = 0;
        for(long x : a){
            total += x;
            curMax = Math.max(x, curMax + x); maxK = Math.max(maxK, curMax);
            curMin = Math.min(x, curMin + x); minK = Math.min(minK, curMin);
        }
        long nonWrap = Math.max(0L, maxK);
        long wrap = total - minK;
        return Math.max(nonWrap, wrap);
    }
    public static void main(String[] args){
        System.out.println(maxCircular(new long[]{8,-1,3,4})); // 15
        System.out.println(maxCircular(new long[]{-4,5,1,0})); // 6
    }
}
