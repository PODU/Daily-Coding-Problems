// Misere Nim: P-position (loss for mover) iff (all heaps==1 with even count) or (some heap>1 and xor==0). First wins otherwise.
// Time O(n), Space O(1).
#include <iostream>
#include <vector>
using namespace std;

bool firstPlayerWins(const vector<int>& heaps) {
    int x = 0, mx = 0;
    for (int h : heaps) { x ^= h; if (h > mx) mx = h; }
    bool pPosition;
    if (mx <= 1) pPosition = (x == 0);          // all heaps == 1: P iff even count
    else pPosition = (x == 0);                   // some heap > 1: P iff xor == 0
    return !pPosition;
}

int main() {
    vector<int> heaps = {3, 4, 5};
    cout << (firstPlayerWins(heaps) ? "true" : "false") << endl; // expected true
    return 0;
}
