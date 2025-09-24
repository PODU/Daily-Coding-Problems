// Merge two binary trees recursively (sum overlaps, keep lone nodes); print merged tree level-order skipping nulls.
// Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left = nullptr, *right = nullptr;
    Node(int v) : val(v) {}
};

Node* merge(Node* a, Node* b) {
    if (!a) return b;
    if (!b) return a;
    Node* n = new Node(a->val + b->val);
    n->left = merge(a->left, b->left);
    n->right = merge(a->right, b->right);
    return n;
}

int main() {
    // Tree1
    Node* t1 = new Node(1);
    t1->left = new Node(3); t1->right = new Node(2);
    t1->left->left = new Node(5);
    // Tree2
    Node* t2 = new Node(2);
    t2->left = new Node(1); t2->right = new Node(3);
    t2->left->right = new Node(4);
    t2->right->right = new Node(7);

    Node* m = merge(t1, t2);

    queue<Node*> q;
    q.push(m);
    while (!q.empty()) {
        int sz = q.size();
        string line;
        for (int i = 0; i < sz; i++) {
            Node* cur = q.front(); q.pop();
            if (i) line += " ";
            line += to_string(cur->val);
            if (cur->left) q.push(cur->left);
            if (cur->right) q.push(cur->right);
        }
        cout << line << "\n";
    }
    return 0;
}
