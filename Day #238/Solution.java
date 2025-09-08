// Blackjack with known deck: deal alternately (player,dealer,player,dealer), then the player
// may take k>=0 hits. Try every k (until bust), simulate the forced dealer, pick the best net
// score (+1 win / 0 push / -1 loss). Time: O(deck), Space: O(1).
public class Solution {
    static int bestScore(int[] deck) {
        int player2 = deck[0] + deck[2];
        int dealer2 = deck[1] + deck[3];
        int best = Integer.MIN_VALUE;
        int psum = player2, idx = 4;
        while (true) {
            int outcome;
            if (psum > 21) {
                outcome = -1;
            } else {
                int dsum = dealer2, di = idx;
                while (dsum <= 16 && di < deck.length) dsum += deck[di++];
                if (dsum > 21 || psum > dsum) outcome = 1;
                else if (psum < dsum) outcome = -1;
                else outcome = 0;
            }
            best = Math.max(best, outcome);
            if (psum > 21 || idx >= deck.length) break;
            psum += deck[idx++];
        }
        return best;
    }

    public static void main(String[] args) {
        int[] deck = {5, 10, 6, 9, 10, 2, 3, 7, 8, 4};
        System.out.println("Best score: " + bestScore(deck)); // 1
    }
}
