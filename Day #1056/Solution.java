// Look-and-say sequence: start "1"; each term describes digit runs of previous.
// Iteratively build N terms by run-length encoding. Time O(N * L), Space O(L).
public class Solution {
    static String lookAndSay(int n) {
        String cur = "1";
        for (int i = 1; i < n; i++) {
            StringBuilder next = new StringBuilder();
            int j = 0;
            while (j < cur.length()) {
                int count = 1;
                while (j + count < cur.length() && cur.charAt(j + count) == cur.charAt(j)) count++;
                next.append(count).append(cur.charAt(j));
                j += count;
            }
            cur = next.toString();
        }
        return cur;
    }

    public static void main(String[] args) {
        int N = 5; // terms: 1, 11, 21, 1211, 111221
        System.out.println(lookAndSay(N));
    }
}
