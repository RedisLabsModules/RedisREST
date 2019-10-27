# -*- coding: utf-8 -*-

from rmtest import BaseModuleTestCase
import rmtest.config
import redis
import unittest
import json
import os

rmtest.config.REDIS_MODULE = os.path.abspath(os.path.join(os.getcwd(), 'target/debug/libredisrest.so'))

class RedisRESTTestCase(BaseModuleTestCase):
    """Tests RedisRET Redis module in vitro"""

    def assertNotExists(self, r, key, msg=None):
        self.assertFalse(r.exists(key), msg)

    def assertOk(self, x, msg=None):
        self.assertEquals("OK", x, msg)

    def assertExists(self, r, key, msg=None):
        self.assertTrue(r.exists(key), msg)

    def testPrepend(self):
        """Test X.PREPEND """
        with self.redis() as r:
            r.client_setname(self._testMethodName)
            r.flushdb()


if __name__ == '__main__':
    unittest.main()
