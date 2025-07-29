// Day 51: Uniform shuffle via Fisher-Yates, using only rand(1..k) and swaps.
// Each of n! permutations equally likely. Time: O(n), Space: O(1).
import java.util.*;

public class Solution {
    static Random rng = new Random();

    // Perfectly random integer in [1, k].
    static int randK(int k) { return rng.nextInt(k) + 1; }

    static void shuffle(int[] deck) {
        for (int i = deck.length - 1; i > 0; i--) {
            int j = randK(i + 1) - 1;   // uniform index in [0, i]
            int tmp = deck[i]; deck[i] = deck[j]; deck[j] = tmp;
        }
    }

    public static void main(String[] args) {
        int[] deck = new int[52];
        for (int i = 0; i < 52; i++) deck[i] = i;
        shuffle(deck);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < deck.length; i++) {
            if (i > 0) sb.append(' ');
            sb.append(deck[i]);
        }
        System.out.println(sb);
        int[] seen = deck.clone();
        Arrays.sort(seen);
        boolean ok = true;
        for (int i = 0; i < 52; i++) if (seen[i] != i) ok = false;
        System.out.println("valid permutation: " + ok);
    }
}
