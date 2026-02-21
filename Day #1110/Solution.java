// Day 1110: Detect a Pythagorean triplet a^2+b^2=c^2 in an array.
// Square all, sort; for each largest c use two-pointer scan. Time: O(N^2). Space: O(N).
import java.util.*;

public class Solution {
    static boolean hasTriplet(int[] a){
        int n = a.length;
        int[] sq = new int[n];
        for (int i = 0; i < n; i++) sq[i] = a[i]*a[i];
        Arrays.sort(sq);
        for (int c = n-1; c >= 2; c--){
            int l = 0, r = c-1;
            while (l < r){
                int s = sq[l] + sq[r];
                if (s == sq[c]) return true;
                if (s < sq[c]) l++; else r--;
            }
        }
        return false;
    }
    public static void main(String[] args){
        System.out.println(hasTriplet(new int[]{3,1,4,6,5}));  // true
        System.out.println(hasTriplet(new int[]{10,4,6,12,5})); // false
    }
}
