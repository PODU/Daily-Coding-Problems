// Day 1852: Longest Increasing Subsequence (strict).
// Patience sorting: maintain tails[]; binary-search insertion point. O(n log n) time, O(n) space.
import java.util.*;

public class Solution {
    static int lis(int[] a) {
        int[] tails = new int[a.length];
        int size = 0;
        for (int x : a) {
            int lo = 0, hi = size;
            while (lo < hi) {
                int mid = (lo + hi) >>> 1;
                if (tails[mid] < x) lo = mid + 1; else hi = mid;
            }
            tails[lo] = x;
            if (lo == size) size++;
        }
        return size;
    }

    public static void main(String[] args) {
        int[] a = {10, 9, 2, 5, 3, 7, 101, 18};
        System.out.println(lis(a)); // 4
    }
}
