// Day 1062: Swap every two adjacent nodes in a singly linked list.
// Approach: iterative pointer manipulation. Time O(n), Space O(1).
#include <iostream>

struct Node {
    int val;
    Node* next;
    Node(int v) : val(v), next(nullptr) {}
};

Node* swapPairs(Node* head) {
    Node dummy(0);
    dummy.next = head;
    Node* prev = &dummy;
    while (prev->next && prev->next->next) {
        Node* a = prev->next;
        Node* b = a->next;
        a->next = b->next;
        b->next = a;
        prev->next = b;
        prev = a;
    }
    return dummy.next;
}

void printList(Node* head) {
    bool first = true;
    while (head) {
        if (!first) std::cout << " -> ";
        std::cout << head->val;
        first = false;
        head = head->next;
    }
    std::cout << "\n";
}

int main() {
    Node* head = new Node(1);
    head->next = new Node(2);
    head->next->next = new Node(3);
    head->next->next->next = new Node(4);
    head = swapPairs(head);
    printList(head); // 2 -> 1 -> 4 -> 3
    return 0;
}
