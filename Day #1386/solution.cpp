// Second largest in BST via reverse in-order (right,node,left), stop at 2nd visited node. O(h) space, O(n) worst time.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left, *right; Node(int v):val(v),left(nullptr),right(nullptr){} };

int secondLargest(Node* root) {
    Node* cur = root;
    stack<Node*> st;
    int count = 0;
    while (cur || !st.empty()) {
        while (cur) { st.push(cur); cur = cur->right; }
        cur = st.top(); st.pop();
        if (++count == 2) return cur->val;
        cur = cur->left;
    }
    return -1; // fewer than 2 nodes
}

int main() {
    Node* root = new Node(5);
    root->left = new Node(3);
    root->left->left = new Node(2);
    root->left->right = new Node(4);
    root->right = new Node(8);
    root->right->left = new Node(7);
    root->right->right = new Node(9);
    cout << secondLargest(root) << "\n";
    return 0;
}
