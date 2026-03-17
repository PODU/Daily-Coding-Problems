// Merge two binary trees recursively (sum overlaps, keep lone nodes). O(min(n1,n2)) time.
// Serialize merged tree as BFS level-order with trailing nulls trimmed.
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

string serialize(Node* root) {
    vector<string> out;
    queue<Node*> q;
    q.push(root);
    while (!q.empty()) {
        Node* n = q.front(); q.pop();
        if (!n) { out.push_back("null"); continue; }
        out.push_back(to_string(n->val));
        q.push(n->left);
        q.push(n->right);
    }
    while (!out.empty() && out.back() == "null") out.pop_back();
    string s = "[";
    for (size_t i = 0; i < out.size(); i++) {
        if (i) s += ", ";
        s += out[i];
    }
    return s + "]";
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

    Node* merged = merge(t1, t2);
    cout << serialize(merged) << "\n";
    return 0;
}
