// Smallest window containing every distinct char: sliding window with counts.
// Time O(n), Space O(k).
import java.util.HashMap;
import java.util.HashSet;
import java.util.Set;

public class Solution {
    static int smallestWindow(String s) {
        Set<Character> distinct = new HashSet<>();
        for (char c : s.toCharArray()) distinct.add(c);
        int need = distinct.size();
        HashMap<Character, Integer> cnt = new HashMap<>();
        int formed = 0, left = 0, best = s.length();
        for (int right = 0; right < s.length(); right++) {
            char c = s.charAt(right);
            cnt.put(c, cnt.getOrDefault(c, 0) + 1);
            if (cnt.get(c) == 1) formed++;
            while (formed == need) {
                best = Math.min(best, right - left + 1);
                char lc = s.charAt(left);
                cnt.put(lc, cnt.get(lc) - 1);
                if (cnt.get(lc) == 0) formed--;
                left++;
            }
        }
        return best;
    }

    public static void main(String[] args) {
        System.out.println(smallestWindow("jiujitsu"));
    }
}
