// Day 412: Nth term of the look-and-say sequence via run-length encoding.
// Build each term from the previous by counting consecutive runs. O(N * L) time where L = term length, O(L) space.
public class Solution {
    static String lookAndSay(int n) {
        String cur = "1";
        for (int t = 1; t < n; t++) {
            StringBuilder next = new StringBuilder();
            int i = 0, m = cur.length();
            while (i < m) {
                int j = i;
                while (j < m && cur.charAt(j) == cur.charAt(i)) j++;
                next.append(j - i).append(cur.charAt(i));
                i = j;
            }
            cur = next.toString();
        }
        return cur;
    }

    public static void main(String[] args) {
        int n = 4;
        System.out.println(lookAndSay(n));
    }
}
