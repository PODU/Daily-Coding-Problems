// Day 284: Find all cousins of a node (same level, different parent) via BFS.
// Time O(N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

vector<int> cousins(Node* root, int target) {
    queue<Node*> q;
    q.push(root);
    while (!q.empty()) {
        int sz = (int)q.size();
        vector<int> level;
        bool found = false;
        // parent of target on this level, to exclude its siblings
        Node* targetParent = nullptr;
        for (int i = 0; i < sz; i++) {
            Node* n = q.front(); q.pop();
            for (Node* c : {n->left, n->right}) {
                if (c) {
                    level.push_back(c->val);
                    if (c->val == target) { found = true; targetParent = n; }
                    q.push(c);
                }
            }
        }
        if (found) {
            vector<int> res;
            // exclude target's siblings (children of targetParent) and target
            set<int> sib;
            if (targetParent->left) sib.insert(targetParent->left->val);
            if (targetParent->right) sib.insert(targetParent->right->val);
            for (int v : level) if (!sib.count(v)) res.push_back(v);
            return res;
        }
    }
    return {};
}

int main() {
    Node* root = new Node(1);
    root->left = new Node(2); root->right = new Node(3);
    root->left->left = new Node(4); root->left->right = new Node(5);
    root->right->right = new Node(6);
    for (int v : cousins(root, 4)) cout << v << " "; // 6
    cout << "\n";
    return 0;
}
