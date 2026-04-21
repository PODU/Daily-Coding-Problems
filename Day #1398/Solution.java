// Single scan: count drops; on a drop, greedily fix by lowering a[i] or
// raising a[i+1] depending on a[i-1]. >1 drop => false. Time O(n), Space O(1).
public class Solution {
    static boolean checkPossibility(int[] a) {
        int cnt = 0;
        for (int i = 0; i + 1 < a.length; i++) {
            if (a[i] > a[i + 1]) {
                if (++cnt > 1) return false;
                if (i == 0 || a[i - 1] <= a[i + 1]) a[i] = a[i + 1];
                else a[i + 1] = a[i];
            }
        }
        return true;
    }

    public static void main(String[] args) {
        System.out.println(checkPossibility(new int[]{10, 5, 7}));
        System.out.println(checkPossibility(new int[]{10, 5, 1}));
    }
}
