// XOR doubly linked list using real pointer arithmetic.
// both = (uintptr_t)prev XOR (uintptr_t)next; traverse with XOR.
// add: O(1), get(i): O(i). Space O(n).
#include <iostream>
#include <cstdint>
using namespace std;

struct Node {
    int value;
    Node* both; // XOR of prev and next addresses
    Node(int v) : value(v), both(nullptr) {}
};

class XorList {
    Node* head = nullptr;
    Node* tail = nullptr;
    static Node* x(Node* a, Node* b) {
        return reinterpret_cast<Node*>(
            reinterpret_cast<uintptr_t>(a) ^ reinterpret_cast<uintptr_t>(b));
    }
public:
    void add(int element) {
        Node* node = new Node(element);
        if (!head) { head = tail = node; return; }
        node->both = x(tail, nullptr);
        tail->both = x(x(tail->both, nullptr), node);
        tail = node;
    }
    int get(int index) {
        Node* prev = nullptr;
        Node* cur = head;
        for (int i = 0; i < index; i++) {
            Node* next = x(prev, cur->both);
            prev = cur;
            cur = next;
        }
        return cur->value;
    }
};

int main() {
    XorList list;
    list.add(10);
    list.add(20);
    list.add(30);
    list.add(40);
    cout << list.get(0) << "\n";
    cout << list.get(3) << "\n";
    return 0;
}
