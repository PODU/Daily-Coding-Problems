// In-order traversal -> sorted array, then two-pointer for pair summing to K. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left, *right; Node(int v):val(v),left(nullptr),right(nullptr){} };

void inorder(Node* r, vector<int>& a) {
    if (!r) return;
    inorder(r->left, a); a.push_back(r->val); inorder(r->right, a);
}

int main() {
    Node* root = new Node(10);
    root->left = new Node(5);
    root->right = new Node(15);
    root->right->left = new Node(11);
    root->right->right = new Node(15);
    int K = 20;
    vector<int> a; inorder(root, a);
    int i = 0, j = (int)a.size() - 1;
    while (i < j) {
        int s = a[i] + a[j];
        if (s == K) { cout << a[i] << " and " << a[j] << "\n"; return 0; }
        else if (s < K) i++;
        else j--;
    }
    cout << "No pair found\n";
    return 0;
}
