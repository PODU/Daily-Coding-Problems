// Mastermind: brute force all 6-permutations of digits 0-9 (10P6=151200),
// keep one consistent with every guess score. Time: O(10P6 * G), Space: O(1).
import java.util.*;

public class Solution {
    static int score(int[] secret, String guess) {
        int s = 0;
        for (int i = 0; i < 6; i++)
            if (secret[i] == guess.charAt(i) - '0') s++;
        return s;
    }

    static boolean consistent(Map<String, Integer> guesses) {
        int[] secret = new int[6];
        boolean[] used = new boolean[10];
        return search(secret, 0, used, guesses);
    }

    static boolean search(int[] secret, int pos, boolean[] used, Map<String, Integer> guesses) {
        if (pos == 6) {
            for (Map.Entry<String, Integer> e : guesses.entrySet())
                if (score(secret, e.getKey()) != e.getValue()) return false;
            return true;
        }
        for (int d = 0; d < 10; d++) {
            if (used[d]) continue;
            used[d] = true;
            secret[pos] = d;
            if (search(secret, pos + 1, used, guesses)) return true;
            used[d] = false;
        }
        return false;
    }

    public static void main(String[] args) {
        Map<String, Integer> ex1 = new LinkedHashMap<>();
        ex1.put("175286", 2); ex1.put("293416", 3); ex1.put("654321", 0);
        Map<String, Integer> ex2 = new LinkedHashMap<>();
        ex2.put("123456", 4); ex2.put("345678", 4); ex2.put("567890", 4);
        System.out.println(consistent(ex1) ? "True" : "False");
        System.out.println(consistent(ex2) ? "True" : "False");
    }
}
