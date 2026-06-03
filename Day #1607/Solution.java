// Bijective char mapping s1->s2: lengths equal + consistent forward map + injective (no two s1 chars share an s2 char).
// Time O(n), Space O(1) (fixed 256-size maps).
public class Solution {
    static boolean bijectiveMap(String s1, String s2) {
        if (s1.length() != s2.length()) return false;
        int[] fwd = new int[256], rev = new int[256];
        java.util.Arrays.fill(fwd, -1);
        java.util.Arrays.fill(rev, -1);
        for (int i = 0; i < s1.length(); i++) {
            int a = s1.charAt(i), b = s2.charAt(i);
            if (fwd[a] == -1 && rev[b] == -1) { fwd[a] = b; rev[b] = a; }
            else if (fwd[a] != b || rev[b] != a) return false;
        }
        return true;
    }

    public static void main(String[] args) {
        System.out.println(bijectiveMap("abc", "bcd"));
        System.out.println(bijectiveMap("foo", "bar"));
    }
}
