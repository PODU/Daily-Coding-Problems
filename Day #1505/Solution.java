// Jump game: greedy, track furthest reachable index.
// Time O(n), Space O(1). Prints "True"/"False" (capitalized) per spec.
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
        int[] a = {2, 0, 1, 0};
        System.out.println(canJump(a) ? "True" : "False");
    }
}
