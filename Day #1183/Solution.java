// Day 1183: Generate all N-digit strobogrammatic numbers (same when rotated 180).
// Recursively build from outside in using rotation pairs; drop leading zeros.
// Time O(output size), Space O(N) recursion depth.
import java.util.*;

public class Solution {
    static final char[][] PAIRS = {{'0','0'},{'1','1'},{'6','9'},{'8','8'},{'9','6'}};

    static List<String> helper(int m) {
        if (m == 0) return new ArrayList<>(List.of(""));
        if (m == 1) return new ArrayList<>(List.of("0", "1", "8"));
        List<String> inner = helper(m - 2), res = new ArrayList<>();
        for (String s : inner)
            for (char[] p : PAIRS)
                res.add(p[0] + s + p[1]);
        return res;
    }

    static List<Long> strobogrammatic(int n) {
        List<Long> out = new ArrayList<>();
        for (String s : helper(n))
            if (!(s.length() > 1 && s.charAt(0) == '0') && !s.equals("0"))
                out.add(Long.parseLong(s));
        Collections.sort(out);
        return out;
    }

    static boolean isStrobo(String s) {
        Map<Character,Character> rot = Map.of('0','0','1','1','6','9','8','8','9','6');
        for (int l = 0, r = s.length() - 1; l <= r; l++, r--)
            if (!rot.containsKey(s.charAt(l)) || rot.get(s.charAt(l)) != s.charAt(r)) return false;
        return true;
    }

    public static void main(String[] args) {
        System.out.println("2-digit strobogrammatic numbers: " + strobogrammatic(2));
        System.out.println("16891 is strobogrammatic: " + isStrobo("16891"));
    }
}
