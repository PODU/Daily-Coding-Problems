// Min flips so all x's precede all y's. Greedy: res=min(res+1, yCount).
// Time O(n), Space O(1).
public class Solution {
    static int minFlips(String s) {
        int res = 0, yCount = 0;
        for (int i = 0; i < s.length(); i++) {
            char ch = s.charAt(i);
            if (ch == 'y') yCount++;
            else res = Math.min(res + 1, yCount);
        }
        return res;
    }

    public static void main(String[] args) {
        System.out.println(minFlips("xyxxxyxyy"));
    }
}
