// Day 596: Invert a binary tree (mirror it).
// Approach: recursively swap left/right children. Time O(n), Space O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    char val;
    Node *left = nullptr, *right = nullptr;
    Node(char v) : val(v) {}
};

void invert(Node* root) {
    if (!root) return;
    swap(root->left, root->right);
    invert(root->left);
    invert(root->right);
}

int main() {
    // Build the example tree:
    //     a
    //    / \
    //   b   c
    //  / \  /
    // d   e f
    Node* a = new Node('a');
    Node* b = new Node('b');
    Node* c = new Node('c');
    Node* d = new Node('d');
    Node* e = new Node('e');
    Node* f = new Node('f');
    a->left = b; a->right = c;
    b->left = d; b->right = e;
    c->left = f;

    invert(a);

    // Level-order print of the mirrored tree.
    queue<Node*> q; q.push(a);
    while (!q.empty()) {
        int sz = q.size();
        string line;
        for (int i = 0; i < sz; i++) {
            Node* n = q.front(); q.pop();
            line += n->val; line += ' ';
            if (n->left) q.push(n->left);
            if (n->right) q.push(n->right);
        }
        if (!line.empty()) line.pop_back();
        cout << line << "\n";
    }
    return 0;
}
