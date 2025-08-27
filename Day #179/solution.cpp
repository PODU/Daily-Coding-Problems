// Reconstruct BST from postorder: iterate right-to-left as (root,right,left) with a lower bound. O(n) time/space.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left=nullptr, *right=nullptr; Node(int v):val(v){} };

int idx;
vector<int> post;

Node* build(int lower){
    if(idx < 0 || post[idx] < lower) return nullptr;
    int val = post[idx]; idx--;
    Node* node = new Node(val);
    node->right = build(val);
    node->left  = build(lower);
    return node;
}

void inorder(Node* n, vector<int>& o){ if(n){ inorder(n->left,o); o.push_back(n->val); inorder(n->right,o); } }
void postorder(Node* n, vector<int>& o){ if(n){ postorder(n->left,o); postorder(n->right,o); o.push_back(n->val); } }

int main(){
    post = {2,4,3,8,7,5};
    idx = (int)post.size()-1;
    Node* root = build(INT_MIN);
    vector<int> ino, po;
    inorder(root, ino); postorder(root, po);
    cout << "Inorder:";   for(int x: ino) cout << " " << x; cout << "\n";
    cout << "Postorder:"; for(int x: po)  cout << " " << x; cout << "\n";
    return 0;
}
