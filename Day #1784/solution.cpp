// Morris in-order traversal: thread tree via predecessor links for O(1) space.
// Time O(N), Space O(1) (excluding output).
#include <iostream>
using namespace std;

struct Node { int val; Node* left; Node* right; Node(int v):val(v),left(nullptr),right(nullptr){} };

void morrisInorder(Node* root) {
    Node* cur = root;
    bool first = true;
    while (cur) {
        if (!cur->left) {
            if (!first) cout << ' ';
            cout << cur->val; first = false;
            cur = cur->right;
        } else {
            Node* pre = cur->left;
            while (pre->right && pre->right != cur) pre = pre->right;
            if (!pre->right) { pre->right = cur; cur = cur->left; }
            else {
                pre->right = nullptr;
                if (!first) cout << ' ';
                cout << cur->val; first = false;
                cur = cur->right;
            }
        }
    }
    cout << '\n';
}

int main() {
    Node* root = new Node(4);
    root->left = new Node(2);
    root->right = new Node(5);
    root->left->left = new Node(1);
    root->left->right = new Node(3);
    morrisInorder(root);
    return 0;
}
