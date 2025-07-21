// Count unival subtrees: postorder; a subtree is unival if both children are
// unival and match the node's value. Time: O(n), Space: O(h).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v, Node* l = nullptr, Node* r = nullptr) : val(v), left(l), right(r) {}
};

int count = 0;
bool isUnival(Node* n) {
    if (!n) return true;
    bool l = isUnival(n->left), r = isUnival(n->right);
    if (!l || !r) return false;
    if (n->left && n->left->val != n->val) return false;
    if (n->right && n->right->val != n->val) return false;
    count++;
    return true;
}

int main() {
    Node* root = new Node(0,
        new Node(1),
        new Node(0, new Node(1, new Node(1), new Node(1)), new Node(0)));
    isUnival(root);
    cout << count << "\n";
    return 0;
}
