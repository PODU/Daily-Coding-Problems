// Falling dominoes: two-pass force accumulation (R adds +, L adds -, decay between).
// Sign of net force decides L/R/.; equal force stays '.'. Time O(n), Space O(n).
public class Solution {
    static String pushDominoes(String d) {
        int n = d.length();
        int[] force = new int[n];
        int f = 0;
        for (int i = 0; i < n; i++) {
            char c = d.charAt(i);
            if (c == 'R') f = n;
            else if (c == 'L') f = 0;
            else f = Math.max(f - 1, 0);
            force[i] += f;
        }
        f = 0;
        for (int i = n - 1; i >= 0; i--) {
            char c = d.charAt(i);
            if (c == 'L') f = n;
            else if (c == 'R') f = 0;
            else f = Math.max(f - 1, 0);
            force[i] -= f;
        }
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < n; i++)
            sb.append(force[i] > 0 ? 'R' : (force[i] < 0 ? 'L' : '.'));
        return sb.toString();
    }

    public static void main(String[] args) {
        System.out.println(pushDominoes(".L.R....L"));
        System.out.println(pushDominoes("..R...L.L"));
    }
}
