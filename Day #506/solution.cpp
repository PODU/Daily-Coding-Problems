// Zigzag rearrange linked list values in a single pass by swapping adjacent
// node values that violate the expected ordering. Time O(n), Space O(1).
#include <iostream>

struct Node {
    int val;
    Node* next;
    Node(int v) : val(v), next(nullptr) {}
};

void zigzag(Node* head) {
    bool less = true; // even index expects list[i] <= list[i+1]
    for (Node* cur = head; cur && cur->next; cur = cur->next) {
        if (less) {
            if (cur->val > cur->next->val) std::swap(cur->val, cur->next->val);
        } else {
            if (cur->val < cur->next->val) std::swap(cur->val, cur->next->val);
        }
        less = !less;
    }
}

int main() {
    int vals[] = {1, 2, 3, 4, 5};
    Node* head = nullptr;
    Node* tail = nullptr;
    for (int v : vals) {
        Node* n = new Node(v);
        if (!head) head = tail = n;
        else { tail->next = n; tail = n; }
    }
    zigzag(head);
    for (Node* cur = head; cur; cur = cur->next) {
        std::cout << cur->val;
        if (cur->next) std::cout << " -> ";
    }
    std::cout << std::endl;
    return 0;
}
