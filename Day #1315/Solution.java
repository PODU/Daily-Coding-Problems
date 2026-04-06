// Look-and-say: build each term by run-length encoding the previous one.
// Time O(sum of term lengths), Space O(length of Nth term).
public class Solution {
    static String lookAndSay(int n) {
        String s = "1";
        for (int i = 1; i < n; i++) {
            StringBuilder next = new StringBuilder();
            for (int j = 0; j < s.length(); ) {
                int k = j;
                while (k < s.length() && s.charAt(k) == s.charAt(j)) k++;
                next.append(k - j).append(s.charAt(j));
                j = k;
            }
            s = next.toString();
        }
        return s;
    }

    public static void main(String[] args) {
        System.out.println(lookAndSay(5)); // 111221
    }
}
