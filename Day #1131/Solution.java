// Blackjack with known deck. DFS over deck index: player hits/stands to maximize
// net (+1/0/-1); dealer then plays fixed rule (hit while <=16). O(deck) states.
public class Solution {
    static int[] deck;

    static int dealerPlay(int p, int playerTotal){
        int dealerTotal = deck[1] + deck[3];
        while(dealerTotal <= 16 && p < deck.length){
            dealerTotal += deck[p]; p++;
        }
        if(dealerTotal > 21) return 1;
        if(playerTotal > dealerTotal) return 1;
        if(playerTotal < dealerTotal) return -1;
        return 0;
    }

    static int playerPlay(int p, int playerTotal){
        int best = dealerPlay(p, playerTotal); // stand
        if(p < deck.length){
            int nt = playerTotal + deck[p];
            int hit = (nt > 21) ? -1 : playerPlay(p + 1, nt);
            best = Math.max(best, hit);
        }
        return best;
    }

    public static void main(String[] args){
        deck = new int[]{10, 7, 5, 9, 6, 4, 10, 2};
        int playerTotal = deck[0] + deck[2];
        System.out.println(playerPlay(4, playerTotal));
    }
}
