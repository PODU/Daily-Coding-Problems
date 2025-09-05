// Day 217: Smallest sparse number (no two adjacent set bits) >= N.
// Approach: scan bits low->high; on adjacent 11 with a 0 above, carry up and clear lower bits.
// Time O(bits) ~ O(log N), much faster than O(N log N). Space O(log N).
import java.util.*;

public class Solution {
    static long nextSparse(long n) {
        if (n <= 0) return 0;
        List<Integer> bits = new ArrayList<>();
        long x = n;
        while (x != 0) { bits.add((int) (x & 1)); x >>= 1; }
        bits.add(0); bits.add(0);
        int sz = bits.size();
        int lastFinal = 0;
        for (int i = 1; i < sz - 1; i++) {
            if (bits.get(i) == 1 && bits.get(i - 1) == 1 && bits.get(i + 1) == 0) {
                bits.set(i + 1, 1);
                for (int j = i; j >= lastFinal; j--) bits.set(j, 0);
                lastFinal = i + 1;
            }
        }
        long res = 0;
        for (int i = sz - 1; i >= 0; i--) res = (res << 1) | bits.get(i);
        return res;
    }

    public static void main(String[] args) {
        System.out.println(nextSparse(22)); // -> 32
        System.out.println(nextSparse(21)); // -> 21
    }
}
