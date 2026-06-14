// Bloom filter: fixed 1000-bit array, 3 hashes via double hashing. add/check.
// add O(k), check O(k); Space O(m bits). check may false-positive, never false-negative.
import java.util.*;
public class Solution {
    static final int M = 1000, K = 3;
    static BitSet bits = new BitSet(M);
    static long h1(String s) { long h = 5381; for (char c : s.toCharArray()) h = h * 33 + c; return h; }
    static long h2(String s) { long h = 0; for (char c : s.toCharArray()) h = h * 131 + c; return h; }
    static int idx(long a, long b, int i) { return (int)(((a + (long)i * b) % M + M) % M); }
    static void add(String s) { long a = h1(s), b = h2(s); for (int i = 0; i < K; i++) bits.set(idx(a, b, i)); }
    static boolean check(String s) {
        long a = h1(s), b = h2(s);
        for (int i = 0; i < K; i++) if (!bits.get(idx(a, b, i))) return false;
        return true;
    }
    public static void main(String[] args) {
        add("apple"); add("banana"); add("cat");
        System.out.println("check apple: " + check("apple"));
        System.out.println("check banana: " + check("banana"));
        System.out.println("check cat: " + check("cat"));
        System.out.println("check dog: " + check("dog"));
    }
}
