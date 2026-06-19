// Next bigger integer with the same number of set bits (Gosper's hack). O(1).
#include <iostream>
using namespace std;

unsigned nextSamePopcount(unsigned n){
    unsigned c = n & (-n);              // lowest set bit
    unsigned r = n + c;                 // ripple the carry
    return r | (((n ^ r) >> 2) / c);    // restore trailing ones
}

int main(){
    unsigned n = 6;                     // 0110
    cout << nextSamePopcount(n) << "\n"; // 9 (1001)
    return 0;
}
