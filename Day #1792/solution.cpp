// Binary tree max path sum: DFS returning max downward gain; track global max = node + max(0,left) + max(0,right).
// Time O(n), Space O(h).
#include <iostream>
#include <algorithm>
#include <climits>
using namespace std;

struct Node { int val; Node* left; Node* right; Node(int v):val(v),left(nullptr),right(nullptr){} };

int best;
int gain(Node* n) {
    if (!n) return 0;
    int l = max(0, gain(n->left));
    int r = max(0, gain(n->right));
    best = max(best, n->val + l + r);
    return n->val + max(l, r);
}
int maxPathSum(Node* root) { best = INT_MIN; gain(root); return best; }

int main() {
    Node* root = new Node(-10);
    root->left = new Node(9);
    root->right = new Node(20);
    root->right->left = new Node(15);
    root->right->right = new Node(7);
    cout << maxPathSum(root) << endl; // expected 42
    return 0;
}
