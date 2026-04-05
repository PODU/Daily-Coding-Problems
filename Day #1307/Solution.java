// Fisher-Yates shuffle using a uniform rand(1..k). Each of N! permutations
// equally likely. Time O(N), Space O(1) extra.
import java.util.*;

public class Solution {
    static Random rng = new Random(12345);
    // uniform random integer in [1, k]
    static int randK(int k) { return rng.nextInt(k) + 1; }

    static void shuffleDeck(int[] deck) {
        for (int i = deck.length - 1; i > 0; --i) {
            int j = randK(i + 1) - 1;          // uniform in [0, i]
            int t = deck[i]; deck[i] = deck[j]; deck[j] = t;
        }
    }

    public static void main(String[] args) {
        int[] deck = new int[52];
        for (int i = 0; i < 52; i++) deck[i] = i + 1;
        shuffleDeck(deck);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < deck.length; i++) {
            sb.append(deck[i]);
            if (i + 1 < deck.length) sb.append(' ');
        }
        System.out.println(sb);
    }
}
