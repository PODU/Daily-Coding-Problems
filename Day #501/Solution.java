// Fisher-Yates shuffle using a rand(k) helper returning [1,k]; O(N) time, O(1) extra space.
// Each of the N! permutations is equally likely. Fixed seed -> reproducible output.
import java.util.*;

public class Solution {
    static Random rng = new Random(12345);

    // Uniform integer in [1, k] using the provided RNG.
    static int randk(int k) {
        return rng.nextInt(k) + 1;
    }

    static void shuffleDeck(int[] arr) {
        int n = arr.length;
        for (int i = n - 1; i >= 1; i--) {
            int j = randk(i + 1) - 1; // index in [0, i]
            int tmp = arr[i];
            arr[i] = arr[j];
            arr[j] = tmp;
        }
    }

    public static void main(String[] args) {
        int[] deck = new int[52];
        for (int i = 0; i < 52; i++) deck[i] = i + 1; // cards 1..52
        shuffleDeck(deck);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < 52; i++) {
            sb.append(deck[i]);
            if (i + 1 < 52) sb.append(' ');
        }
        System.out.println(sb.toString());
    }
}
