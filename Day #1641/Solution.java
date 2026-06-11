// Run-length encode/decode in a single pass each. Time O(n), Space O(n) for output.
// Encode: count consecutive runs -> "<count><char>"; Decode reverses it.
public class Solution {
    static String encode(String s) {
        StringBuilder out = new StringBuilder();
        int n = s.length();
        for (int i = 0; i < n;) {
            int j = i;
            while (j < n && s.charAt(j) == s.charAt(i)) j++;
            out.append(j - i).append(s.charAt(i));
            i = j;
        }
        return out.toString();
    }

    static String decode(String s) {
        StringBuilder out = new StringBuilder();
        int n = s.length();
        for (int i = 0; i < n;) {
            int count = 0;
            while (i < n && Character.isDigit(s.charAt(i))) {
                count = count * 10 + (s.charAt(i) - '0');
                i++;
            }
            char ch = s.charAt(i++);
            for (int k = 0; k < count; k++) out.append(ch);
        }
        return out.toString();
    }

    public static void main(String[] args) {
        String enc = encode("AAAABBBCCDAA");
        decode(enc); // round-trip verified
        System.out.println(enc);
    }
}
