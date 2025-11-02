// Reconstruct BST from postorder: scan postorder right-to-left (acts as preorder of
// root,right,left) using an upper-bound recursion. Time O(n), space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left=nullptr,*right=nullptr; Node(int v):val(v){} };

int idx;
Node* build(const vector<int>& post, int bound) {
    if (idx < 0 || post[idx] < bound) return nullptr;
    Node* root = new Node(post[idx--]);
    root->right = build(post, root->val);
    root->left  = build(post, bound);
    return root;
}

void preorder(Node* n, vector<int>& out){ if(!n)return; out.push_back(n->val); preorder(n->left,out); preorder(n->right,out); }
void inorder(Node* n, vector<int>& out){ if(!n)return; inorder(n->left,out); out.push_back(n->val); inorder(n->right,out); }

int main(){
    vector<int> post = {2,4,3,8,7,5};
    idx = (int)post.size()-1;
    Node* root = build(post, INT_MIN);
    vector<int> pre, in;
    preorder(root, pre); inorder(root, in);
    cout << "preorder:";  for(int x:pre) cout << ' ' << x; cout << "\n";
    cout << "inorder: "; for(int x:in)  cout << ' ' << x; cout << "\n";
    return 0;
}
