// Day 1480: Sort a million ints in [0, 1e9] without a billion-element array.
// Index by element count, not by value. LSD radix sort (base 256) is O(N).
// For out-of-core data the same idea generalizes to external merge sort.
// Time O(N) (4 passes for 32-bit), Space O(N).
import java.util.*;

public class Solution {
    static int[] radixSort(int[] in) {
        if (in.length == 0) return in;
        int[] out = in.clone();
        int[] tmp = new int[out.length];
        for (int shift = 0; shift < 32; shift += 8) {
            int[] count = new int[257];
            for (int v : out) count[((v >>> shift) & 0xFF) + 1]++;
            for (int i = 0; i < 256; ++i) count[i + 1] += count[i];
            for (int v : out) {
                int d = (v >>> shift) & 0xFF;
                tmp[count[d]++] = v;
            }
            int[] t = out; out = tmp; tmp = t;
        }
        return out;
    }

    public static void main(String[] args) {
        System.out.println(Arrays.toString(radixSort(new int[]{9, 11, 8, 5, 7, 10})));
        // [5, 7, 8, 9, 10, 11]
    }
}
