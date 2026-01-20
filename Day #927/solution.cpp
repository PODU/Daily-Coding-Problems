// Rotate list right by k: find length L, make a ring, break at L-(k%L).
// Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node* next;
    Node(int v) : val(v), next(nullptr) {}
};

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

Node* rotateRight(Node* head, int k) {
    if (!head || !head->next) return head;
    int L = 1;
    Node* tail = head;
    while (tail->next) { tail = tail->next; L++; }
    k %= L;
    if (k == 0) return head;
    tail->next = head; // ring
    int steps = L - k;
    Node* newTail = head;
    for (int i = 0; i < steps - 1; i++) newTail = newTail->next;
    Node* newHead = newTail->next;
    newTail->next = nullptr;
    return newHead;
}

int main() {
    Node* head = build({1, 2, 3, 4, 5});
    cout << toStr(rotateRight(head, 3)) << endl;
    return 0;
}
