// Level-order (BFS) traversal of a binary tree using a queue. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node* left;
    Node* right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

vector<int> levelOrder(Node* root) {
    vector<int> res;
    if (!root) return res;
    queue<Node*> q;
    q.push(root);
    while (!q.empty()) {
        Node* cur = q.front(); q.pop();
        res.push_back(cur->val);
        if (cur->left) q.push(cur->left);
        if (cur->right) q.push(cur->right);
    }
    return res;
}

int main() {
    Node* root = new Node(1);
    root->left = new Node(2);
    root->right = new Node(3);
    root->right->left = new Node(4);
    root->right->right = new Node(5);

    vector<int> vals = levelOrder(root);
    for (size_t i = 0; i < vals.size(); ++i) {
        if (i) cout << ", ";
        cout << vals[i];
    }
    cout << "\n";
    return 0;
}
