// Day 275: Nth term of look-and-say (term 1 = "1").
// Build each term by run-length encoding the previous. Time O(N * len), Space O(len).
public class Solution {
    static String lookAndSay(int n) {
        String cur = "1";
        for (int t = 1; t < n; t++) {
            StringBuilder nxt = new StringBuilder();
            int i = 0, m = cur.length();
            while (i < m) {
                int j = i;
                while (j < m && cur.charAt(j) == cur.charAt(i)) j++;
                nxt.append(j - i).append(cur.charAt(i));
                i = j;
            }
            cur = nxt.toString();
        }
        return cur;
    }

    public static void main(String[] args) {
        System.out.println(lookAndSay(5)); // 111221
    }
}
