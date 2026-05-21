// Fisher-Yates shuffle: uniform random permutation using only swaps.
// rand(k) gives a uniform int in [1,k]; each of N! orderings is equally likely.
// Time O(N), Space O(1).
import java.util.*;

public class Solution {
    static Random rng = new Random(12345);
    // uniform random number between 1 and k inclusive
    static int randk(int k) { return rng.nextInt(k) + 1; }

    static void shuffle(int[] deck) {
        for (int i = deck.length - 1; i > 0; i--) {
            int j = randk(i + 1) - 1; // index in [0, i]
            int t = deck[i]; deck[i] = deck[j]; deck[j] = t;
        }
    }

    public static void main(String[] args) {
        int[] deck = new int[52];
        for (int i = 0; i < 52; i++) deck[i] = i + 1;
        shuffle(deck);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < deck.length; i++) {
            if (i > 0) sb.append(' ');
            sb.append(deck[i]);
        }
        System.out.println(sb);
    }
}
