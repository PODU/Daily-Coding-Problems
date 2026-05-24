// Lazy binary tree: generate() returns a root in O(1) whose children are thunks
// (std::function) forced on demand; a seeded coin flip (<1 continue prob) makes the
// tree finite w.p.1 but unbounded. Realization is depth-capped for a deterministic demo.
#include <bits/stdc++.h>
using namespace std;

struct LCG {
    unsigned long long s;
    LCG(unsigned long long seed) : s(seed) {}
    unsigned long long next() { s = (s * 16807ULL) % 2147483647ULL; return s; } // Park-Miller
    bool coin() { return next() % 100 < 45; } // child exists with prob 0.45 -> finite
};

struct Node {
    int val;
    function<shared_ptr<Node>()> left;
    function<shared_ptr<Node>()> right;
};

// makeNode/generate do NOT force children: O(1).
shared_ptr<Node> makeNode(shared_ptr<LCG> rng, shared_ptr<int> counter) {
    auto node = make_shared<Node>();
    node->val = (*counter)++;
    node->left = [rng, counter]() -> shared_ptr<Node> {
        return rng->coin() ? makeNode(rng, counter) : nullptr;
    };
    node->right = [rng, counter]() -> shared_ptr<Node> {
        return rng->coin() ? makeNode(rng, counter) : nullptr;
    };
    return node;
}

shared_ptr<Node> generate(shared_ptr<LCG> rng, shared_ptr<int> counter) {
    return makeNode(rng, counter); // O(1): one node, children unevaluated
}

int realize(shared_ptr<Node> node, int depth, int cap) {
    int count = 1;
    if (depth < cap) {
        auto l = node->left();
        if (l) count += realize(l, depth + 1, cap);
        auto r = node->right();
        if (r) count += realize(r, depth + 1, cap);
    }
    return count;
}

int main() {
    auto rng = make_shared<LCG>(42);
    auto counter = make_shared<int>(0);
    auto root = generate(rng, counter); // returns instantly
    cout << "generate() returned a lazy tree root in O(1)" << endl;
    int n = realize(root, 0, 6);
    cout << "Realized tree node count: " << n << endl;
    return 0;
}
