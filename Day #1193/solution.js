// Ghost word game: trie + game theory. canWin(node)=mover can force win.
// Winning start c: child not a word AND opponent (canWin(child)) cannot win.
// Time O(total chars), Space O(total chars).
// NOTE: README sample shows only "b" but "c" is also winning.

function makeNode() {
    return { ch: new Map(), isWord: false };
}

function insert(root, w) {
    let cur = root;
    for (const c of w) {
        if (!cur.ch.has(c)) cur.ch.set(c, makeNode());
        cur = cur.ch.get(c);
    }
    cur.isWord = true;
}

// can the player about to move from this prefix force a win?
function canWin(node) {
    for (const child of node.ch.values()) {
        if (child.isWord) continue;       // completing a word loses
        if (!canWin(child)) return true;  // opponent loses
    }
    return false;
}

function main() {
    const dict = ["cat", "calf", "dog", "bear"];
    const root = makeNode();
    for (const w of dict) insert(root, w);

    const wins = [];
    for (const [c, child] of root.ch.entries()) {
        if (!child.isWord && !canWin(child)) wins.push(c);
    }
    wins.sort();
    console.log(wins.join(" "));
}

main();
