// Day 1576: Blackjack solver with perfect deck knowledge.
// Player enumerates how many cards to hit (stops if bust); dealer plays forced rules.
// Time: O(N^2) total; Space: O(1).
public class Solution {
    static int dealerPlay(int[] deck, int idx, int dealerTotal) {
        while (dealerTotal <= 16 && idx < deck.length) dealerTotal += deck[idx++];
        return dealerTotal;
    }

    static int compareScore(int player, int dealer) {
        if (player > 21) return -1;
        if (dealer > 21) return 1;
        if (player > dealer) return 1;
        if (player < dealer) return -1;
        return 0;
    }

    static int bestScore(int[] deck) {
        int playerTotal = deck[0] + deck[1];
        int best = Integer.MIN_VALUE;
        int k = 0;
        while (true) {
            if (playerTotal <= 21) {
                int dealer = dealerPlay(deck, 4 + k, deck[2] + deck[3]);
                best = Math.max(best, compareScore(playerTotal, dealer));
            } else break;
            if (4 + k >= deck.length) break;
            playerTotal += deck[4 + k];
            k++;
        }
        return best;
    }

    public static void main(String[] args) {
        int[] deck = {10, 6, 9, 7, 5, 10, 2};
        System.out.println("Optimal player score: " + bestScore(deck));
    }
}
