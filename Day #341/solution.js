// Max non-overlapping dictionary words on a board.
// (1) DFS enumerate every placement (bitmask of cells) per word. (2) Backtrack over
// placements choosing pairwise-disjoint sets to maximize count.
'use strict';

function solve(board, dictionary){
    const R = board.length, C = board[0].length;

    function dfs(w, idx, r, c, used, masks){
        if(board[r][c] !== w[idx]) return;
        const cell = r*C + c;
        if(used & (1<<cell)) return;
        used |= (1<<cell);
        if(idx === w.length-1){ masks.add(used); return; }
        const dirs = [[-1,0],[1,0],[0,-1],[0,1]];
        for(const [dr,dc] of dirs){
            const nr=r+dr, nc=c+dc;
            if(nr>=0&&nr<R&&nc>=0&&nc<C) dfs(w, idx+1, nr, nc, used, masks);
        }
    }

    const placements = []; // [wordIndex, mask]
    for(let wi=0; wi<dictionary.length; wi++){
        const masks = new Set();
        for(let r=0;r<R;r++) for(let c=0;c<C;c++) dfs(dictionary[wi],0,r,c,0,masks);
        for(const m of masks) placements.push([wi, m]);
    }

    let best = 0;
    function backtrack(start, occupied, usedWords, count){
        if(count > best) best = count;
        for(let i=start;i<placements.length;i++){
            const [w, m] = placements[i];
            if(usedWords & (1<<w)) continue;
            if(occupied & m) continue;
            backtrack(i+1, occupied|m, usedWords|(1<<w), count+1);
        }
    }
    backtrack(0,0,0,0);
    return best;
}

function main(){
    const dictionary = ['eat','rain','in','rat'];
    const board = [['e','a','n'],['t','t','i'],['a','r','a']];
    console.log(solve(board, dictionary));
}

main();
