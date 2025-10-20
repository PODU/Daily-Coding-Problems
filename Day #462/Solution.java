// Day 462: Smallest sparse number (no adjacent set bits) >= N.
// Scan bits low->high, lift each "11" pair upward. Time O(log N), Space O(log N).
import java.util.ArrayList;
import java.util.List;

public class Solution {
    static long nextSparse(long n) {
        if (n <= 0) return n;
        List<Integer> bits = new ArrayList<>();
        for (long t = n; t > 0; t >>= 1) bits.add((int) (t & 1));
        bits.add(0); bits.add(0);
        int size = bits.size(), lastFinal = 0;
        for (int i = 1; i < size - 1; i++) {
            if (bits.get(i) == 1 && bits.get(i - 1) == 1 && bits.get(i + 1) == 0) {
                bits.set(i + 1, 1);
                for (int j = i; j >= lastFinal; j--) bits.set(j, 0);
                lastFinal = i + 1;
            }
        }
        long ans = 0;
        for (int i = 0; i < size; i++) if (bits.get(i) == 1) ans |= (1L << i);
        return ans;
    }

    public static void main(String[] args) {
        System.out.println(nextSparse(22)); // 32
        System.out.println(nextSparse(21)); // 21
    }
}
