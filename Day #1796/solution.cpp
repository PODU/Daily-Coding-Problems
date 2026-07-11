// Uniform random in [0,n-1] excluding values in l: precompute sorted allowed array, pick random index. O(n) build, O(allowed) space.
#include <iostream>
#include <vector>
#include <set>
#include <random>

class RandomPicker {
    std::vector<int> allowed;
public:
    RandomPicker(int n, const std::vector<int>& l) {
        std::set<int> ex(l.begin(), l.end());
        for (int i = 0; i < n; ++i)
            if (ex.find(i) == ex.end()) allowed.push_back(i);
    }
    int pick() {
        static std::mt19937 rng(12345);
        std::uniform_int_distribution<size_t> d(0, allowed.size() - 1);
        return allowed[d(rng)];
    }
    const std::vector<int>& getAllowed() const { return allowed; }
};

int main() {
    RandomPicker rp(10, {2, 3, 5, 7});
    const auto& a = rp.getAllowed();
    std::set<int> aset(a.begin(), a.end());

    std::cout << "allowed=[";
    for (size_t i = 0; i < a.size(); ++i)
        std::cout << a[i] << (i + 1 < a.size() ? ", " : "");
    std::cout << "]\n";

    bool ok = true;
    for (int i = 0; i < 1000; ++i)
        if (aset.find(rp.pick()) == aset.end()) ok = false;
    std::cout << "sample in allowed: " << (ok ? "true" : "false") << "\n";
    return 0;
}
