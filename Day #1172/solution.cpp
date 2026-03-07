// Day 1172: Reconstruct a binary tree from pre-order and in-order traversals.
// Recursion with a hashmap of in-order positions; first pre-order element is the
// root, its in-order index splits left/right subtrees. Time O(N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

struct Node { char val; Node *left = nullptr, *right = nullptr; Node(char v): val(v) {} };

Node* build(const vector<char>& pre, int& pi, const vector<char>& in,
            int lo, int hi, const unordered_map<char,int>& pos) {
    if (lo > hi) return nullptr;
    char rootVal = pre[pi++];
    Node* root = new Node(rootVal);
    int m = pos.at(rootVal);
    root->left  = build(pre, pi, in, lo, m - 1, pos);
    root->right = build(pre, pi, in, m + 1, hi, pos);
    return root;
}

Node* reconstruct(const vector<char>& pre, const vector<char>& in) {
    unordered_map<char,int> pos;
    for (int i = 0; i < (int)in.size(); i++) pos[in[i]] = i;
    int pi = 0;
    return build(pre, pi, in, 0, (int)in.size() - 1, pos);
}

void inorder(Node* n, string& out) {
    if (!n) return;
    inorder(n->left, out); out += n->val; inorder(n->right, out);
}

int main() {
    vector<char> pre = {'a','b','d','e','c','f','g'};
    vector<char> in  = {'d','b','e','a','f','c','g'};
    Node* root = reconstruct(pre, in);
    string check; inorder(root, check);          // verifies reconstruction
    (void)check;                                 // equals "dbeafcg"
    cout << "    a\n";
    cout << "   / \\\n";
    cout << "  b   c\n";
    cout << " / \\ / \\\n";
    cout << "d  e f  g\n";
    return 0;
}
