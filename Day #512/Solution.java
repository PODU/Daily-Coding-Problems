// Jump Game: greedily track the furthest reachable index. O(n) time, O(1) space.
public class Solution {
    static boolean canJump(int[] a) {
        int reach = 0;
        for (int i = 0; i < a.length; i++) {
            if (i > reach) return false;
            reach = Math.max(reach, i + a[i]);
        }
        return true;
    }

    public static void main(String[] args) {
        System.out.println(canJump(new int[]{1, 3, 1, 2, 0, 1}));
        System.out.println(canJump(new int[]{1, 2, 1, 0, 0}));
    }
}
