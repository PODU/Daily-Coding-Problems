// XOR linked list using real pointer XOR (uintptr_t). add=O(1), get(i)=O(i) time, O(1) extra space.
#include <iostream>
#include <cstdint>
using namespace std;

struct Node {
    int val;
    uintptr_t both; // XOR of prev and next addresses
    Node(int v) : val(v), both(0) {}
};

class XorList {
    Node* head = nullptr;
    Node* tail = nullptr;
public:
    void add(int v) {
        Node* n = new Node(v);
        if (!head) { head = tail = n; return; }
        n->both = (uintptr_t)tail;                 // prev=tail, next=NULL(0)
        tail->both ^= (uintptr_t)n;                 // append n as next of old tail
        tail = n;
    }
    int get(int index) {
        uintptr_t prev = 0;
        Node* cur = head;
        for (int i = 0; i < index; ++i) {
            uintptr_t next = cur->both ^ prev;
            prev = (uintptr_t)cur;
            cur = (Node*)next;
        }
        return cur->val;
    }
};

int main() {
    XorList list;
    list.add(10); list.add(20); list.add(30); list.add(40);
    cout << list.get(0) << "\n" << list.get(1) << "\n"
         << list.get(2) << "\n" << list.get(3) << "\n";
    return 0;
}
