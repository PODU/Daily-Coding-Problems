# Day 726: Singleton holding TWO instances; alternate on even/odd getInstance() calls.
# Approach: Class-level counter; odd call -> instance #2, even call -> instance #1.
# Time: O(1) per call, Space: O(1).

class DualSingleton:
    _first = None
    _second = None
    _count = 0

    def __init__(self, id):
        self.id = id

    @classmethod
    def get_instance(cls):
        if cls._first is None:
            cls._first = DualSingleton(1)
            cls._second = DualSingleton(2)
        cls._count += 1
        return cls._first if cls._count % 2 == 0 else cls._second  # even->first, odd->second


if __name__ == "__main__":
    for i in range(1, 5):
        print(f"Call {i}: instance {DualSingleton.get_instance().id}")
