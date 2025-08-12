// Day 109: Swap adjacent bit pairs: ((x&0xAA)>>1)|((x&0x55)<<1). O(1).
#include <bits/stdc++.h>
using namespace std;
unsigned char swapBits(unsigned char x){
    return ((x & 0xAA) >> 1) | ((x & 0x55) << 1);
}
string toBin(unsigned char x){ return bitset<8>(x).to_string(); }
int main(){
    cout << toBin(swapBits((unsigned char)0b10101010)) << "\n"; // 01010101
    cout << toBin(swapBits((unsigned char)0b11100010)) << "\n"; // 11010001
    return 0;
}
