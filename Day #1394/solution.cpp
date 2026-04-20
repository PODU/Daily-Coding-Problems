// Reverse singly linked list in-place: iterative 3-pointer (prev/cur/next). O(n) time, O(1) space.
#include <iostream>

struct Node {
    int val;
    Node* next;
    Node(int v) : val(v), next(nullptr) {}
};

Node* reverseList(Node* head) {
    Node* prev = nullptr;
    Node* cur = head;
    while (cur) {
        Node* next = cur->next;
        cur->next = prev;
        prev = cur;
        cur = next;
    }
    return prev;
}

int main() {
    Node* head = new Node(1);
    head->next = new Node(2);
    head->next->next = new Node(3);
    head->next->next->next = new Node(4);
    head->next->next->next->next = new Node(5);

    head = reverseList(head);

    for (Node* p = head; p; p = p->next) {
        std::cout << p->val;
        if (p->next) std::cout << " -> ";
    }
    std::cout << std::endl;
    return 0;
}
