# -*- coding: utf-8 -*-

from rmtest import BaseModuleTestCase
import rmtest.config
import redis
import unittest
import json
import os

rmtest.config.REDIS_MODULE = os.path.abspath(os.path.join(os.getcwd(), 'target/debug/libredisx.so'))

class RedisXTestCase(BaseModuleTestCase):
    """Tests RedisX Redis module in vitro"""

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

            with self.assertRaises(redis.exceptions.ResponseError):
                r.execute_command('x.prepend', 'key')

            with self.assertRaises(redis.exceptions.ResponseError):
                r.execute_command('x.prepend', 'key', 'abc', 'dfc')

            self.assertEquals(3, r.execute_command('x.prepend', 'key1', 'abc'))
            self.assertEquals('abc', r.get('key1'))

            self.assertEquals(6, r.execute_command('x.prepend', 'key1', 'def'))
            self.assertEquals('defabc', r.get('key1'))

if __name__ == '__main__':
    unittest.main()
