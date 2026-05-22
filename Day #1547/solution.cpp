// XOR linked list simulated with a vector of nodes indexed by integer addresses.
// add appends in O(1); get traverses with nextAddr = curBoth XOR prevAddr in O(index). Space O(n).
#include <iostream>
#include <vector>
using namespace std;

struct Node { int val; int both; }; // both = prevAddr XOR nextAddr

struct XorList {
    vector<Node> mem; // mem[0] unused as sentinel; addresses start at 1
    int head = 0, tail = 0;
    XorList() { mem.push_back({0, 0}); }
    void add(int v) {
        int addr = (int)mem.size();
        mem.push_back({v, 0});
        if (head == 0) { head = tail = addr; }
        else {
            mem[tail].both ^= addr;   // tail's next becomes addr
            mem[addr].both ^= tail;   // new node's prev is old tail
            tail = addr;
        }
    }
    int get(int index) {
        int prev = 0, cur = head;
        for (int i = 0; i < index; i++) {
            int next = mem[cur].both ^ prev;
            prev = cur; cur = next;
        }
        return mem[cur].val;
    }
};

int main() {
    XorList list;
    for (int v : {10, 20, 30, 40, 50}) list.add(v);
    cout << list.get(0) << "\n";
    cout << list.get(2) << "\n";
    cout << list.get(4) << "\n";
    return 0;
}
