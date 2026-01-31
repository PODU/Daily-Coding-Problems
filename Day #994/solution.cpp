// Day 994: Print binary tree nodes level by level (BFS).
// Use a queue; dequeue a node, emit it, enqueue its children. O(n) time/space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v, Node* l = nullptr, Node* r = nullptr) : val(v), left(l), right(r) {}
};

vector<int> levelOrder(Node* root) {
    vector<int> out;
    queue<Node*> q;
    if (root) q.push(root);
    while (!q.empty()) {
        Node* n = q.front(); q.pop();
        out.push_back(n->val);
        if (n->left) q.push(n->left);
        if (n->right) q.push(n->right);
    }
    return out;
}

int main() {
    Node* root = new Node(1, new Node(2), new Node(3, new Node(4), new Node(5)));
    vector<int> v = levelOrder(root);
    for (size_t i = 0; i < v.size(); ++i) cout << v[i] << (i + 1 < v.size() ? ", " : "\n");
    return 0;
}
