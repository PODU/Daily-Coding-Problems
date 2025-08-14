// Day 118: Two-pointer merge from both ends into result back-to-front. O(n) time, O(n) space.
import java.util.*;

public class Solution {
    static int[] sortedSquares(int[] a){
        int n = a.length; int[] res = new int[n];
        int lo = 0, hi = n - 1;
        for (int k = n - 1; k >= 0; k--){
            int sl = a[lo] * a[lo], sh = a[hi] * a[hi];
            if (sl > sh){ res[k] = sl; lo++; } else { res[k] = sh; hi--; }
        }
        return res;
    }
    public static void main(String[] args){
        System.out.println(Arrays.toString(sortedSquares(new int[]{-9,-2,0,2,3})));
    }
}
