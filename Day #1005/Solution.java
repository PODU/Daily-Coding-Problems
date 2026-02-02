// Day 1005: Blackjack solver with perfect knowledge of the deck order.
// Player gets deck[0],deck[2]; dealer gets deck[1],deck[3]; rest is the draw pile.
// Try every number of player hits k, keep best outcome; dealer hits while <= 16.
// O(N^2) over the deck.
public class Solution {
    static int bestScore(int[] deck) {
        int n = deck.length;
        int playerBase = deck[0] + deck[2];
        int dealerBase = deck[1] + deck[3];
        int best = -1;
        for (int k = 0; ; k++) {
            int player = playerBase;
            for (int i = 0; i < k; i++) player += deck[4 + i];
            if (player > 21) break;
            int idx = 4 + k, dealer = dealerBase;
            while (dealer <= 16 && idx < n) dealer += deck[idx++];
            int outcome = (dealer > 21 || player > dealer) ? 1 : (player < dealer ? -1 : 0);
            best = Math.max(best, outcome);
            if (4 + k >= n) break;
        }
        return best;
    }

    public static void main(String[] args) {
        int[] deck = {10, 10, 6, 9, 5, 7, 3, 8};
        System.out.println("Best player score: " + bestScore(deck)); // 1
    }
}
