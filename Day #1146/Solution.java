// Day 1146: Dominoes - two-pass force accumulation.
// L->R pass adds rightward force, R->L pass adds leftward; sign decides. O(n) time, O(n) space.
public class Solution {
    static String pushDominoes(String s) {
        int n = s.length();
        int[] forces = new int[n];
        int force = 0;
        for (int i = 0; i < n; i++) {
            char c = s.charAt(i);
            if (c == 'R') force = n; else if (c == 'L') force = 0; else force = Math.max(force - 1, 0);
            forces[i] += force;
        }
        force = 0;
        for (int i = n - 1; i >= 0; i--) {
            char c = s.charAt(i);
            if (c == 'L') force = n; else if (c == 'R') force = 0; else force = Math.max(force - 1, 0);
            forces[i] -= force;
        }
        StringBuilder sb = new StringBuilder();
        for (int f : forces) sb.append(f > 0 ? 'R' : f < 0 ? 'L' : '.');
        return sb.toString();
    }

    public static void main(String[] args) {
        System.out.println(pushDominoes(".L.R....L")); // LL.RRRLLL
        System.out.println(pushDominoes("..R...L.L")); // ..RR.LLLL
    }
}
