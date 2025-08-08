// Day 83: Invert (mirror) a binary tree by swapping children recursively.
// Time O(n), Space O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    char val; Node *left, *right;
    Node(char v) : val(v), left(nullptr), right(nullptr) {}
};

Node* invert(Node* root) {
    if (!root) return nullptr;
    swap(root->left, root->right);
    invert(root->left);
    invert(root->right);
    return root;
}

string levelOrder(Node* root) {
    string out; queue<Node*> q;
    if (root) q.push(root);
    while (!q.empty()) {
        Node* n = q.front(); q.pop();
        out += n->val; out += ' ';
        if (n->left) q.push(n->left);
        if (n->right) q.push(n->right);
    }
    if (!out.empty()) out.pop_back();
    return out;
}

int main() {
    Node* a = new Node('a');
    a->left = new Node('b'); a->right = new Node('c');
    a->left->left = new Node('d'); a->left->right = new Node('e');
    a->right->left = new Node('f');
    cout << "before: " << levelOrder(a) << "\n"; // a b c d e f
    invert(a);
    cout << "after:  " << levelOrder(a) << "\n"; // a c b f e d
    return 0;
}
