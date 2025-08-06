// Longest strictly increasing subsequence via patience sorting (tails array + binary search).
// Time O(n log n), Space O(n).
import java.util.Arrays;

public class Solution {
    static int lengthOfLIS(int[] a) {
        int[] tails = new int[a.length];
        int len = 0;
        for (int x : a) {
            int i = Arrays.binarySearch(tails, 0, len, x);
            if (i < 0) i = -(i + 1);
            tails[i] = x;
            if (i == len) len++;
        }
        return len;
    }

    public static void main(String[] args) {
        int[] a = {0,8,4,12,2,10,6,14,1,9,5,13,3,11,7,15};
        System.out.println(lengthOfLIS(a));
    }
}
