// Day 48: Reconstruct binary tree from preorder + inorder.
// Hashmap of inorder positions; recurse. Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node { char val; Node *left = nullptr, *right = nullptr; Node(char v): val(v) {} };

Node* build(const vector<char>& pre, int& preIdx, int inL, int inR,
            const unordered_map<char,int>& pos) {
    if (inL > inR) return nullptr;
    char rootVal = pre[preIdx++];
    Node* root = new Node(rootVal);
    int mid = pos.at(rootVal);
    root->left = build(pre, preIdx, inL, mid - 1, pos);
    root->right = build(pre, preIdx, mid + 1, inR, pos);
    return root;
}

int main() {
    vector<char> pre = {'a','b','d','e','c','f','g'};
    vector<char> in  = {'d','b','e','a','f','c','g'};
    unordered_map<char,int> pos;
    for (int i = 0; i < (int)in.size(); i++) pos[in[i]] = i;
    int preIdx = 0;
    Node* root = build(pre, preIdx, 0, in.size() - 1, pos);

    // Level-order traversal confirms reconstruction: a b c d e f g
    queue<Node*> q; q.push(root);
    bool first = true;
    while (!q.empty()) {
        Node* n = q.front(); q.pop();
        cout << (first ? "" : " ") << n->val; first = false;
        if (n->left) q.push(n->left);
        if (n->right) q.push(n->right);
    }
    cout << endl;
    return 0;
}
