// Day 1803: Singleton holding two instances; getInstance alternates first (even call) / second (odd call).
// Lazy init via function-local statics; O(1) per call, O(1) space.
#include <iostream>

class DualSingleton {
public:
    int id;
    static DualSingleton& getInstance() {
        static DualSingleton first(1);   // created once, on first call
        static DualSingleton second(2);
        long c = counter++;              // call count starts at 0
        return (c % 2 == 0) ? first : second;
    }
private:
    explicit DualSingleton(int i) : id(i) {}
    static long counter;
};
long DualSingleton::counter = 0;

int main() {
    DualSingleton* prevEven = nullptr;
    for (int i = 0; i < 4; ++i) {
        DualSingleton& inst = DualSingleton::getInstance();
        std::cout << "call" << i << "->" << inst.id << "\n";
        if (i % 2 == 0) {
            if (prevEven) std::cout << "  even-call identity same: "
                << (prevEven == &inst ? "true" : "false") << "\n";
            prevEven = &inst;
        }
    }
    return 0;
}
