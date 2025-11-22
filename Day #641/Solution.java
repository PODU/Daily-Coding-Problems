// Day 641: Smallest positive integer not expressible as a subset sum.
// Approach: scan sorted array; if a[i] > reach+1 a gap exists, else reach += a[i].
// Time: O(N), Space: O(1).
public class Solution {
    static long smallestNonSum(long[] a) {
        long reach = 0; // all of [1..reach] are representable
        for (long x : a) {
            if (x > reach + 1) break;
            reach += x;
        }
        return reach + 1;
    }

    public static void main(String[] args) {
        long[] a = {1, 2, 3, 10};
        System.out.println(smallestNonSum(a)); // 7
    }
}
