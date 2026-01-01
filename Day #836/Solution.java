// Day 836: Fisher-Yates shuffle using only a rand(k) RNG (uniform 1..k) and swaps.
// For i=n-1..1: pick j uniform in 0..i via rand(i+1)-1, swap a[i],a[j]. O(N) time, O(1) extra.
// Unbiased: each step picks uniformly among i+1 positions, so all n! permutations are equally likely.
import java.util.*;

public class Solution {
    static class RNG {
        long state;
        RNG(long seed) { state = seed; }
        long next() {
            state = state * 6364136223846793005L + 1442695040888963407L;
            return state >>> 16;
        }
        // Uniform in [1, k] with rejection to avoid modulo bias.
        long rand(long k) {
            long mask = (1L << 48) - 1;
            long limit = (1L << 48) - ((1L << 48) % k);
            while (true) {
                long r = next() & mask;
                if (r < limit) return r % k + 1;
            }
        }
    }

    static void shuffle(int[] a, RNG rng) {
        for (int i = a.length - 1; i > 0; i--) {
            int j = (int) rng.rand(i + 1) - 1; // uniform 0..i
            int t = a[i]; a[i] = a[j]; a[j] = t;
        }
    }

    public static void main(String[] args) {
        int[] deck = new int[52];
        for (int i = 0; i < 52; i++) deck[i] = i + 1;
        shuffle(deck, new RNG(12345));
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < deck.length; i++) {
            sb.append(deck[i]);
            if (i + 1 < deck.length) sb.append(' ');
        }
        System.out.println(sb.toString());
    }
}
