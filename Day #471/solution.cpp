// Generate all structurally distinct BSTs with values 1..N via recursive root selection.
// Time: O(Catalan(N) * N), Space: O(Catalan(N) * N) for the trees.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node* left;
    Node* right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

vector<Node*> generate(int start, int end) {
    vector<Node*> trees;
    if (start > end) {
        trees.push_back(nullptr);
        return trees;
    }
    for (int i = start; i <= end; ++i) {
        vector<Node*> lefts = generate(start, i - 1);
        vector<Node*> rights = generate(i + 1, end);
        for (Node* l : lefts) {
            for (Node* r : rights) {
                Node* root = new Node(i);
                root->left = l;
                root->right = r;
                trees.push_back(root);
            }
        }
    }
    return trees;
}

void preorder(Node* root, vector<int>& out) {
    if (!root) return;
    out.push_back(root->val);
    preorder(root->left, out);
    preorder(root->right, out);
}

int main() {
    int N = 3;
    vector<Node*> trees = generate(1, N);
    cout << "Number of BSTs: " << trees.size() << "\n";
    for (Node* t : trees) {
        vector<int> out;
        preorder(t, out);
        for (size_t i = 0; i < out.size(); ++i) {
            if (i) cout << " ";
            cout << out[i];
        }
        cout << "\n";
    }
    return 0;
}
