// Reconstruct binary tree from preorder+inorder using inorder index hashmap
// and a moving preorder pointer. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node { char val; Node* left; Node* right; Node(char v):val(v),left(nullptr),right(nullptr){} };

unordered_map<char,int> idx;
int preIdx;

Node* build(const vector<char>& pre, int inL, int inR) {
    if (inL > inR) return nullptr;
    char rootVal = pre[preIdx++];
    Node* root = new Node(rootVal);
    int mid = idx[rootVal];
    root->left  = build(pre, inL, mid - 1);
    root->right = build(pre, mid + 1, inR);
    return root;
}

void preorder(Node* n, vector<char>& out){ if(!n)return; out.push_back(n->val); preorder(n->left,out); preorder(n->right,out); }
void inorder(Node* n, vector<char>& out){ if(!n)return; inorder(n->left,out); out.push_back(n->val); inorder(n->right,out); }
void postorder(Node* n, vector<char>& out){ if(!n)return; postorder(n->left,out); postorder(n->right,out); out.push_back(n->val); }

string join(const vector<char>& v){ string s; for(size_t i=0;i<v.size();++i){ if(i) s+=' '; s+=v[i]; } return s; }

int main(){
    vector<char> pre = {'a','b','d','e','c','f','g'};
    vector<char> in  = {'d','b','e','a','f','c','g'};
    for(int i=0;i<(int)in.size();++i) idx[in[i]] = i;
    preIdx = 0;
    Node* root = build(pre, 0, (int)in.size()-1);

    vector<char> po, pr, io;
    postorder(root, po);
    preorder(root, pr);
    inorder(root, io);
    cout << "postorder: " << join(po) << "\n";
    cout << "preorder:  " << join(pr) << "\n";
    cout << "inorder:   " << join(io) << "\n";
    return 0;
}
