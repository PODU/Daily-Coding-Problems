# Day 1435: Singleton holding two instances; even call -> first, odd call -> second.
# Approach: Two stored instances + class-level call counter, return by parity of count.
# Time: O(1) per call, Space: O(1).


class DualSingleton:
    _first = None
    _second = None
    _counter = 0

    def __init__(self, id):
        self.id = id

    @classmethod
    def get_instance(cls):
        if cls._first is None:
            cls._first = DualSingleton(1)
            cls._second = DualSingleton(2)
        n = cls._counter            # 0-indexed call number
        cls._counter += 1
        return cls._first if n % 2 == 0 else cls._second


if __name__ == "__main__":
    for i in range(4):
        inst = DualSingleton.get_instance()
        print(f"Call {i} -> instance {inst.id}")
    # Even calls (0,2) -> instance 1, odd calls (1,3) -> instance 2
