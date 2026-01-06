// Day 863: generate() returns a finite but arbitrarily large binary tree in O(1).
// Approach: root created in O(1); children expanded lazily with randomized termination
// (each child exists with prob < 0.5 => branching process is finite almost surely).
// generate(): O(1). Materializing the whole tree: O(size). Deterministic demo via MINSTD RNG.
#include <bits/stdc++.h>
using namespace std;

long long rngState = 42;
double nextRand() {
    rngState = (rngState * 48271) % 2147483647;
    return (double)rngState / 2147483647.0;
}

struct Node {
    Node *left = nullptr, *right = nullptr;
    bool expanded = false;
};

const double P = 0.45;
const int DEPTH_CAP = 50;

// Lazily create children on first access.
void ensureChildren(Node* n, int depth) {
    if (n->expanded) return;
    n->expanded = true;
    if (depth >= DEPTH_CAP) return;
    if (nextRand() < P) n->left = new Node();
    if (nextRand() < P) n->right = new Node();
}

Node* generate() { return new Node(); }   // O(1)

int countNodes(Node* n, int depth) {
    if (!n) return 0;
    ensureChildren(n, depth);
    return 1 + countNodes(n->left, depth + 1) + countNodes(n->right, depth + 1);
}

int main() {
    Node* root = generate();
    cout << "Generated a finite binary tree with " << countNodes(root, 0)
         << " nodes (deterministic demo)." << endl;
    return 0;
}
