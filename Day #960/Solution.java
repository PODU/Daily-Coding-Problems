// Day 960: jump game - can we reach the last index? Greedy furthest-reach.
// Time O(n), Space O(1).
public class Solution {
    static boolean canReach(int[] a) {
        int reach = 0;
        for (int i = 0; i < a.length; i++) {
            if (i > reach) return false;
            reach = Math.max(reach, i + a[i]);
        }
        return true;
    }

    public static void main(String[] args) {
        System.out.println(canReach(new int[]{1, 3, 1, 2, 0, 1})); // true
        System.out.println(canReach(new int[]{1, 2, 1, 0, 0}));    // false
    }
}
