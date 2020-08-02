import os
import unittest

import procmaps


class TestProcmaps(unittest.TestCase):
    def check_map_properties(self, map_):
        self.assertIsInstance(map_.begin_address, int)
        self.assertIsInstance(map_.end_address, int)

        self.assertTrue(map_.begin_address in map_)
        self.assertFalse(map_.end_address in map_)

        self.assertIsInstance(map_.is_readable, bool)
        self.assertIsInstance(map_.is_writable, bool)
        self.assertIsInstance(map_.is_executable, bool)
        self.assertIsInstance(map_.is_shared, bool)
        self.assertIsInstance(map_.is_private, bool)
        self.assertIsInstance(map_.offset, int)
        self.assertIsInstance(map_.device, tuple)
        self.assertIsInstance(map_.device[0], int)
        self.assertIsInstance(map_.device[1], int)
        self.assertIsInstance(map_.inode, int)

        if map_.is_shared:
            self.assertFalse(map_.is_private)

        if map_.is_private:
            self.assertFalse(map_.is_shared)

        self.assertTrue(isinstance(map_.pathname, str) or map_.pathname is None)

    def test_from_pid(self):
        maps = procmaps.from_pid(os.getpid())
        for map_ in maps:
            self.check_map_properties(map_)

    def test_from_path(self):
        maps = procmaps.from_path("/proc/self/maps")
        for map_ in maps:
            self.check_map_properties(map_)

    def test_from_str(self):
        maps = procmaps.from_str("55d5564b4000-55d5564b6000 r--p 00000000 08:11 6553896 /bin/cat")
        self.assertEqual(len(maps), 1)
        self.check_map_properties(maps[0])


if __name__ == "__main__":
    unittest.main()
