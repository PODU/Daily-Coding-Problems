// Day 848: LRU cache with O(1) get/set using a hash map + doubly linked list.
// Map key -> list node; most-recent at front, evict from back. O(1) per op.
#include <bits/stdc++.h>
using namespace std;

class LRUCache {
    int cap;
    list<pair<int,int>> dll;                         // front = MRU
    unordered_map<int, list<pair<int,int>>::iterator> mp;
public:
    LRUCache(int n): cap(n) {}
    int get(int key){
        auto it = mp.find(key);
        if(it == mp.end()) return -1;                // -1 represents null
        dll.splice(dll.begin(), dll, it->second);
        return it->second->second;
    }
    void set(int key, int value){
        auto it = mp.find(key);
        if(it != mp.end()){
            it->second->second = value;
            dll.splice(dll.begin(), dll, it->second);
            return;
        }
        if((int)dll.size() == cap){
            auto last = dll.back();
            mp.erase(last.first);
            dll.pop_back();
        }
        dll.push_front({key, value});
        mp[key] = dll.begin();
    }
};

int main(){
    LRUCache c(2);
    c.set(1,1);
    c.set(2,2);
    cout << c.get(1) << "\n";   // 1
    c.set(3,3);                 // evicts key 2
    cout << c.get(2) << "\n";   // -1 (null)
    c.set(4,4);                 // evicts key 1
    cout << c.get(1) << "\n";   // -1 (null)
    cout << c.get(3) << "\n";   // 3
    cout << c.get(4) << "\n";   // 4
    return 0;
}
