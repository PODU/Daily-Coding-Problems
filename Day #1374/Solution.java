// Jump game: greedy track furthest reachable index. Time O(n), Space O(1).
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
        System.out.println(canReach(new int[]{2, 0, 1, 0}) ? "True" : "False");
        System.out.println(canReach(new int[]{1, 1, 0, 1}) ? "True" : "False");
    }
}
