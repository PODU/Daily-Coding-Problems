# Day 1803: Singleton holding two instances; getInstance alternates first (even call) / second (odd call).
# Lazy init guarded by a lock; O(1) per call, O(1) space.
import threading


class DualSingleton:
    _first = None
    _second = None
    _counter = 0
    _lock = threading.Lock()

    def __init__(self, _id):
        self.id = _id

    @classmethod
    def get_instance(cls):
        with cls._lock:
            if cls._first is None:
                cls._first = DualSingleton(1)
                cls._second = DualSingleton(2)
            c = cls._counter          # call count starts at 0
            cls._counter += 1
            return cls._first if c % 2 == 0 else cls._second


def main():
    prev_even = None
    for i in range(4):
        inst = DualSingleton.get_instance()
        print(f"call{i}->{inst.id}")
        if i % 2 == 0:
            if prev_even is not None:
                print(f"  even-call identity same: {str(prev_even is inst).lower()}")
            prev_even = inst


if __name__ == "__main__":
    main()
