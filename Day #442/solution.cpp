// Day 442: Cartesian tree (min-heap ordered, in-order == S) built with a
// monotonic stack in O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left = nullptr, *right = nullptr; Node(int v):val(v){} };

Node* buildCartesian(const vector<int>& s) {
    vector<Node*> stk;
    for (int v : s) {
        Node* node = new Node(v);
        Node* last = nullptr;
        while (!stk.empty() && stk.back()->val > v) { last = stk.back(); stk.pop_back(); }
        node->left = last;
        if (!stk.empty()) stk.back()->right = node;
        stk.push_back(node);
    }
    return stk.empty() ? nullptr : stk.front();
}

void print(Node* n, string prefix, string tag) {
    if (!n) return;
    cout << prefix << tag << n->val << "\n";
    print(n->left,  prefix + "  ", "L-- ");
    print(n->right, prefix + "  ", "R-- ");
}

int main() {
    vector<int> s = {3, 2, 6, 1, 9};
    Node* root = buildCartesian(s);
    print(root, "", "");
    // 1
    //   L-- 2
    //     L-- 3
    //     R-- 6
    //   R-- 9
    return 0;
}
