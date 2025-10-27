// Day 503: Sort a singly linked list using bottom-up (iterative) merge sort.
// Time O(n log n), Space O(1) auxiliary (no recursion).
#include <iostream>

struct Node {
    int val;
    Node* next;
    Node(int v) : val(v), next(nullptr) {}
};

int listLength(Node* head) {
    int n = 0;
    for (; head; head = head->next) ++n;
    return n;
}

// Split off `size` nodes starting at head; cut the list there and return the rest.
Node* split(Node* head, int size) {
    for (int i = 1; head && i < size; ++i) head = head->next;
    if (!head) return nullptr;
    Node* rest = head->next;
    head->next = nullptr;
    return rest;
}

// Merge two sorted lists, attach result after `tail`, return the new tail.
Node* mergeLists(Node* a, Node* b, Node* tail) {
    while (a && b) {
        if (a->val <= b->val) { tail->next = a; a = a->next; }
        else { tail->next = b; b = b->next; }
        tail = tail->next;
    }
    tail->next = a ? a : b;
    while (tail->next) tail = tail->next;
    return tail;
}

Node* sortList(Node* head) {
    if (!head || !head->next) return head;
    int n = listLength(head);
    Node dummy(0);
    dummy.next = head;
    for (int size = 1; size < n; size *= 2) {
        Node* tail = &dummy;
        Node* cur = dummy.next;
        while (cur) {
            Node* left = cur;
            Node* right = split(left, size);
            cur = split(right, size);
            tail = mergeLists(left, right, tail);
        }
    }
    return dummy.next;
}

void printList(Node* head) {
    bool first = true;
    for (; head; head = head->next) {
        if (!first) std::cout << " -> ";
        std::cout << head->val;
        first = false;
    }
    std::cout << std::endl;
}

int main() {
    int vals[] = {4, 1, -3, 99};
    Node dummy(0);
    Node* tail = &dummy;
    for (int v : vals) { tail->next = new Node(v); tail = tail->next; }
    Node* sorted = sortList(dummy.next);
    printList(sorted); // -3 -> 1 -> 4 -> 99
    return 0;
}
