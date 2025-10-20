// Reverse a singly linked list in-place by re-pointing each next pointer.
// Time: O(n), Space: O(1).
#include <iostream>
using namespace std;

struct Node {
    int val;
    Node* next;
    Node(int v) : val(v), next(nullptr) {}
};

Node* reverse(Node* head) {
    Node* prev = nullptr;
    while (head) {
        Node* nxt = head->next;
        head->next = prev;
        prev = head;
        head = nxt;
    }
    return prev;
}

void print(Node* head) {
    bool first = true;
    while (head) {
        if (!first) cout << ' ';
        cout << head->val;
        first = false;
        head = head->next;
    }
    cout << '\n';
}

int main() {
    Node* head = new Node(1);
    head->next = new Node(2);
    head->next->next = new Node(3);
    head->next->next->next = new Node(4);
    head->next->next->next->next = new Node(5);
    print(head);
    head = reverse(head);
    print(head);
    return 0;
}
