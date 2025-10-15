// Day 435: Reconstruct a binary tree from preorder + inorder traversals.
// Approach: recurse, using a hashmap of inorder value->index to find roots in O(1).
// Time: O(n), Space: O(n).
#include <iostream>
#include <vector>
#include <string>
#include <unordered_map>
#include <queue>
using namespace std;

struct Node {
    string val;
    Node *left = nullptr, *right = nullptr;
    Node(string v) : val(v) {}
};

Node* build(const vector<string>& pre, int& pi, int inL, int inR,
            unordered_map<string,int>& idx) {
    if (inL > inR) return nullptr;
    string rootVal = pre[pi++];
    Node* root = new Node(rootVal);
    int mid = idx[rootVal];
    root->left = build(pre, pi, inL, mid - 1, idx);
    root->right = build(pre, pi, mid + 1, inR, idx);
    return root;
}

int main() {
    vector<string> preorder = {"a","b","d","e","c","f","g"};
    vector<string> inorder  = {"d","b","e","a","f","c","g"};
    unordered_map<string,int> idx;
    for (int i = 0; i < (int)inorder.size(); ++i) idx[inorder[i]] = i;
    int pi = 0;
    Node* root = build(preorder, pi, 0, inorder.size() - 1, idx);

    // Print level-order to show reconstruction: a b c d e f g
    queue<Node*> q; q.push(root);
    bool first = true;
    while (!q.empty()) {
        Node* n = q.front(); q.pop();
        if (!first) cout << " ";
        cout << n->val; first = false;
        if (n->left) q.push(n->left);
        if (n->right) q.push(n->right);
    }
    cout << endl;
    return 0;
}
