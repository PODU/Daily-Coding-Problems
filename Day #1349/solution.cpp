// Day 1349: Reconstruct a BST from its postorder traversal.
// Consume postorder right-to-left with value bounds (right subtree before left). O(n) time, O(h) space.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left=nullptr, *right=nullptr; Node(int v):val(v){} };

int idx;
vector<int> post;

Node* build(long long bound) {
    if (idx < 0 || post[idx] < bound) return nullptr;
    Node* node = new Node(post[idx--]);
    node->right = build(node->val);
    node->left = build(bound);
    return node;
}

void preorder(Node* n, vector<int>& out){ if(!n)return; out.push_back(n->val); preorder(n->left,out); preorder(n->right,out);}
void inorder(Node* n, vector<int>& out){ if(!n)return; inorder(n->left,out); out.push_back(n->val); inorder(n->right,out);}

int main() {
    post = {2, 4, 3, 8, 7, 5};
    idx = (int)post.size() - 1;
    Node* root = build(LLONG_MIN);
    vector<int> pre, in;
    preorder(root, pre); inorder(root, in);
    cout << "Root: " << root->val << "\n";
    cout << "Preorder: "; for (int x : pre) cout << x << " "; cout << "\n";
    cout << "Inorder: "; for (int x : in) cout << x << " "; cout << "\n";
    return 0;
}
