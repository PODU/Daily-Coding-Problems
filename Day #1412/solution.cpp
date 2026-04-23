// Day 1412: generate() returns a finite but arbitrarily large binary tree in O(1).
// Approach: lazy nodes — children are materialized only on access (here via a deterministic
// LCG so the demo is reproducible). generate() itself is O(1); a child appears on first touch.
#include <bits/stdc++.h>
using namespace std;

static unsigned long long lcg = 42ULL;
int nextRand() { lcg = lcg * 1103515245ULL + 12345ULL; return (int)((lcg >> 16) & 0x7fff); }

struct LazyNode {
    int depth;
    bool leftDone = false, rightDone = false;
    LazyNode* leftCache = nullptr;
    LazyNode* rightCache = nullptr;
    LazyNode(int d) : depth(d) {}

    bool spawn() { int bound = 55 - depth * 7; return bound > 0 && (nextRand() % 100) < bound; }
    LazyNode* left()  { if (!leftDone)  { leftDone = true;  if (spawn()) leftCache = new LazyNode(depth + 1); } return leftCache; }
    LazyNode* right() { if (!rightDone) { rightDone = true; if (spawn()) rightCache = new LazyNode(depth + 1); } return rightCache; }
};

// generate(): O(1) — just hands back a root; the tree unfolds lazily.
LazyNode* generate() { return new LazyNode(0); }

int countNodes(LazyNode* n) {
    if (!n) return 0;
    int l = countNodes(n->left());   // forces left first (deterministic order)
    int r = countNodes(n->right());
    return 1 + l + r;
}

int main() {
    LazyNode* root = generate();
    cout << "Tree size: " << countNodes(root) << "\n";
    return 0;
}
