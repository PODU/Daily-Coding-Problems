// Blackjack with known deck. DFS over deck index: player hits/stands to maximize
// net (+1/0/-1); dealer then plays fixed rule (hit while <=16). O(deck) states.
'use strict';

const deck = [10, 7, 5, 9, 6, 4, 10, 2];

function dealerPlay(p, playerTotal){
    let dealerTotal = deck[1] + deck[3];
    while(dealerTotal <= 16 && p < deck.length){
        dealerTotal += deck[p]; p++;
    }
    if(dealerTotal > 21) return 1;
    if(playerTotal > dealerTotal) return 1;
    if(playerTotal < dealerTotal) return -1;
    return 0;
}

function playerPlay(p, playerTotal){
    let best = dealerPlay(p, playerTotal); // stand
    if(p < deck.length){
        const nt = playerTotal + deck[p];
        const hit = (nt > 21) ? -1 : playerPlay(p + 1, nt);
        best = Math.max(best, hit);
    }
    return best;
}

function main(){
    const playerTotal = deck[0] + deck[2];
    console.log(playerPlay(4, playerTotal));
}

main();
