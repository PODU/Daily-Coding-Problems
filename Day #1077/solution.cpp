// K-ary tree symmetry: recursively mirror-match children list. O(n) time/space.
#include <iostream>
#include <vector>
#include <memory>

struct Node {
    int val;
    std::vector<std::shared_ptr<Node>> children;
    Node(int v) : val(v) {}
};

bool isMirror(std::shared_ptr<Node> L, std::shared_ptr<Node> R) {
    if (!L && !R) return true;
    if (!L || !R || L->val != R->val) return false;
    int n = L->children.size();
    if ((int)R->children.size() != n) return false;
    for (int i = 0; i < n; i++)
        if (!isMirror(L->children[i], R->children[n - 1 - i])) return false;
    return true;
}

bool isSymmetric(std::shared_ptr<Node> root) {
    if (!root) return true;
    int n = root->children.size();
    for (int i = 0; i < n / 2; i++)
        if (!isMirror(root->children[i], root->children[n - 1 - i])) return false;
    return true;
}

int main() {
    // Symmetric tree: root=4, children=[3,5,3], 3->9, 3->9
    auto root = std::make_shared<Node>(4);
    auto c1 = std::make_shared<Node>(3); c1->children.push_back(std::make_shared<Node>(9));
    auto c2 = std::make_shared<Node>(5);
    auto c3 = std::make_shared<Node>(3); c3->children.push_back(std::make_shared<Node>(9));
    root->children = {c1, c2, c3};
    std::cout << "Symmetric: " << (isSymmetric(root) ? "true" : "false") << "\n";

    // Asymmetric: root=1, children=[2,3]
    auto r2 = std::make_shared<Node>(1);
    r2->children.push_back(std::make_shared<Node>(2));
    r2->children.push_back(std::make_shared<Node>(3));
    std::cout << "Symmetric: " << (isSymmetric(r2) ? "true" : "false") << "\n";
}
