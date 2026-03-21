// B is a rotation of A iff same length and B is a substring of A+A.
// Time O(n), Space O(n).
public class Solution {
    static boolean isRotation(String a, String b) {
        return a.length() == b.length() && (a + a).contains(b);
    }

    public static void main(String[] args) {
        System.out.println(isRotation("abcde", "cdeab"));
        System.out.println(isRotation("abc", "acb"));
    }
}
