// Rotate singly linked list right by k: form ring, break at (len - k%len). Time O(n), Space O(1).
#include <iostream>
#include <vector>
#include <string>
using namespace std;

struct Node {
    int val;
    Node* next;
    Node(int v) : val(v), next(nullptr) {}
};

Node* rotateRight(Node* head, int k) {
    if (!head || !head->next || k == 0) return head;
    int length = 1;
    Node* tail = head;
    while (tail->next) { tail = tail->next; length++; }
    k %= length;
    if (k == 0) return head;
    tail->next = head; // make ring
    int steps = length - k;
    Node* newTail = head;
    for (int i = 0; i < steps - 1; i++) newTail = newTail->next;
    Node* newHead = newTail->next;
    newTail->next = nullptr;
    return newHead;
}

Node* build(const vector<int>& vals) {
    Node dummy(0);
    Node* cur = &dummy;
    for (int v : vals) { cur->next = new Node(v); cur = cur->next; }
    return dummy.next;
}

string toStr(Node* head) {
    string s;
    while (head) {
        s += to_string(head->val);
        if (head->next) s += " -> ";
        head = head->next;
    }
    return s;
}

int main() {
    cout << toStr(rotateRight(build({7, 7, 3, 5}), 2)) << endl;
    cout << toStr(rotateRight(build({1, 2, 3, 4, 5}), 3)) << endl;
    return 0;
}
