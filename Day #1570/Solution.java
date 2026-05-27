// Look-and-say: build each term by run-length encoding the previous. Time O(total digits), space O(len).
public class Solution {
    static String lookAndSay(int n) {
        String cur = "1";
        for (int k = 1; k < n; k++) {
            StringBuilder next = new StringBuilder();
            int i = 0, len = cur.length();
            while (i < len) {
                int j = i;
                while (j < len && cur.charAt(j) == cur.charAt(i)) j++;
                next.append(j - i).append(cur.charAt(i));
                i = j;
            }
            cur = next.toString();
        }
        return cur;
    }

    public static void main(String[] args) {
        System.out.println(lookAndSay(4)); // 1, 11, 21, 1211
    }
}
