// Dominoes final state via two-pass force summation. Time: O(n), Space: O(n).
public class Solution {
    static String dominoes(String s) {
        int n = s.length();
        int[] forces = new int[n];
        // Left to right: R pushes right
        int f = 0;
        for (int i = 0; i < n; i++) {
            if      (s.charAt(i)=='R') f = n;
            else if (s.charAt(i)=='L') f = 0;
            else f = Math.max(0, f-1);
            forces[i] += f;
        }
        // Right to left: L pushes left (subtract)
        f = 0;
        for (int i = n-1; i >= 0; i--) {
            if      (s.charAt(i)=='L') f = n;
            else if (s.charAt(i)=='R') f = 0;
            else f = Math.max(0, f-1);
            forces[i] -= f;
        }
        StringBuilder sb = new StringBuilder();
        for (int v : forces) sb.append(v>0?'R':v<0?'L':'.');
        return sb.toString();
    }

    public static void main(String[] args) {
        System.out.println(dominoes(".L.R....L"));
        System.out.println(dominoes("..R...L.L"));
    }
}
