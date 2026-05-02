// Invert binary tree by swapping children recursively; print level-order (BFS).
// Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    char val;
    Node *left, *right;
    Node(char v): val(v), left(nullptr), right(nullptr) {}
};

void invert(Node* root) {
    if (!root) return;
    swap(root->left, root->right);
    invert(root->left);
    invert(root->right);
}

int main() {
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

    vector<char> out;
    queue<Node*> q;
    q.push(a);
    while (!q.empty()) {
        Node* n = q.front(); q.pop();
        if (!n) continue;
        out.push_back(n->val);
        q.push(n->left);
        q.push(n->right);
    }
    for (size_t i = 0; i < out.size(); ++i) {
        if (i) cout << ' ';
        cout << out[i];
    }
    cout << '\n';
    return 0;
}
