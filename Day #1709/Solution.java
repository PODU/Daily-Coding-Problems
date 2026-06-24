// Non-decreasing with <=1 change: single pass, on violation greedily lower a[i-1] or raise a[i]. O(n).
public class Solution {
    static boolean checkPossibility(int[] a) {
        int cnt = 0;
        for (int i = 1; i < a.length; i++) {
            if (a[i - 1] > a[i]) {
                if (++cnt > 1) return false;
                if (i < 2 || a[i - 2] <= a[i]) a[i - 1] = a[i];
                else a[i] = a[i - 1];
            }
        }
        return true;
    }

    public static void main(String[] args) {
        System.out.println(checkPossibility(new int[]{10, 5, 7}));
        System.out.println(checkPossibility(new int[]{10, 5, 1}));
    }
}
