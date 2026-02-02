// King-in-check: locate K, test rook/queen lines, bishop/queen diagonals, knight, king, pawn.
// Row 0 is top; white pawns attack upward (toward smaller row). Time O(64), Space O(1).
'use strict';

const inBoard = (r, c) => r >= 0 && r < 8 && c >= 0 && c < 8;

function isCheck(b) {
    let kr = -1, kc = -1;
    for (let r = 0; r < 8; r++)
        for (let c = 0; c < 8; c++)
            if (b[r][c] === 'K') { kr = r; kc = c; }

    for (const [dr, dc] of [[1,0],[-1,0],[0,1],[0,-1]]) {
        let r = kr + dr, c = kc + dc;
        while (inBoard(r, c)) {
            const p = b[r][c];
            if (p !== '.') { if (p === 'R' || p === 'Q') return true; break; }
            r += dr; c += dc;
        }
    }
    for (const [dr, dc] of [[1,1],[1,-1],[-1,1],[-1,-1]]) {
        let r = kr + dr, c = kc + dc;
        while (inBoard(r, c)) {
            const p = b[r][c];
            if (p !== '.') { if (p === 'B' || p === 'Q') return true; break; }
            r += dr; c += dc;
        }
    }
    for (const [dr, dc] of [[1,2],[1,-2],[-1,2],[-1,-2],[2,1],[2,-1],[-2,1],[-2,-1]]) {
        const r = kr + dr, c = kc + dc;
        if (inBoard(r, c) && b[r][c] === 'N') return true;
    }
    for (let dr = -1; dr <= 1; dr++)
        for (let dc = -1; dc <= 1; dc++) {
            if (dr === 0 && dc === 0) continue;
            const r = kr + dr, c = kc + dc;
            if (inBoard(r, c) && b[r][c] === 'K') return true;
        }
    for (const dc of [-1, 1]) {
        const r = kr + 1, c = kc + dc;
        if (inBoard(r, c) && b[r][c] === 'P') return true;
    }
    return false;
}

const board = [
    "...K....",
    "........",
    ".B......",
    "......P.",
    ".......R",
    "..N.....",
    "........",
    ".....Q.."
];
console.log(isCheck(board) ? "True" : "False"); // True
