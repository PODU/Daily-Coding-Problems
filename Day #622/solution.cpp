// Deepest node in a binary tree via BFS level order; last visited node is deepest.
// Time O(N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    char val;
    Node *left, *right;
    Node(char v) : val(v), left(nullptr), right(nullptr) {}
};

char deepest(Node* root) {
    if (!root) return '\0';
    queue<Node*> q;
    q.push(root);
    Node* last = root;
    while (!q.empty()) {
        last = q.front(); q.pop();
        if (last->left) q.push(last->left);
        if (last->right) q.push(last->right);
    }
    return last->val;
}

int main() {
    // a children b,c ; b has left child d
    Node* a = new Node('a');
    Node* b = new Node('b');
    Node* c = new Node('c');
    Node* d = new Node('d');
    a->left = b; a->right = c;
    b->left = d;
    cout << deepest(a) << endl; // d
    return 0;
}
