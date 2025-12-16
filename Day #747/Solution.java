// B is a rotation of A iff |A|==|B| and B is a substring of A+A.
// Time: O(n) (String.contains is linear-ish); KMP would guarantee O(n).
public class Solution {
    static boolean isRotation(String a, String b){
        if(a.length() != b.length()) return false;
        return (a + a).contains(b);
    }

    public static void main(String[] args){
        System.out.println(isRotation("abcde", "cdeab")); // true
        System.out.println(isRotation("abc", "acb"));     // false
    }
}
