// Day 599: Ghost game - find starting letters that guarantee player 1 a win.
// Approach: build a trie, minimax over it (landing on a word loses). Time O(total chars), Space O(trie).
'use strict';

class Trie {
    constructor() {
        this.ch = new Map();
        this.isWord = false;
    }
}

function insert(root, w) {
    let node = root;
    for (const c of w) {
        if (!node.ch.has(c)) node.ch.set(c, new Trie());
        node = node.ch.get(c);
    }
    node.isWord = true;
}

// True if the player to move from `node` can force the opponent to lose.
function moverWins(node) {
    for (const child of node.ch.values()) {
        if (child.isWord) continue;
        if (!moverWins(child)) return true;
    }
    return false;
}

function main() {
    const dict = ["cat", "calf", "dog", "bear"];
    const root = new Trie();
    for (const w of dict) insert(root, w);

    const winning = [];
    for (const c of [...root.ch.keys()].sort()) {
        const child = root.ch.get(c);
        if (!child.isWord && !moverWins(child)) winning.push(c);
    }
    console.log(winning.join(' '));
}

main();
