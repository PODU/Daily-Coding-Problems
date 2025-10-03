// Day 364: Longest strictly increasing subsequence length.
// Patience sorting: keep tails[], binary-search the first tail >= x and replace.
// Time O(n log n), Space O(n).
import java.util.*;

public class Solution {
    static int lis(int[] a) {
        int[] tails = new int[a.length];
        int len = 0;
        for (int x : a) {
            int lo = 0, hi = len;
            while (lo < hi) {
                int mid = (lo + hi) >>> 1;
                if (tails[mid] < x) lo = mid + 1;
                else hi = mid;
            }
            tails[lo] = x;
            if (lo == len) len++;
        }
        return len;
    }

    public static void main(String[] args) {
        int[] a = {10, 9, 2, 5, 3, 7, 101, 18};
        System.out.println(lis(a)); // 4
    }
}
