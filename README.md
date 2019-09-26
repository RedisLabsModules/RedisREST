[![GitHub issues](https://img.shields.io/github/release/RedisLabsModules/RedisX.svg)](https://github.com/RedisLabsModules/RedisX/releases/latest)
[![CircleCI](https://circleci.com/gh/RedisLabsModules/RedisX/tree/master.svg?style=svg)](https://circleci.com/gh/RedisLabsModules/RedisX/tree/master)

# RedisX
Extension modules to Redis' native data types and commands




# Commands

## X.PREPEND key value
If key already exists and is a string, this command prepend the value at the begin of the string. If key does not exist it is created and set as an empty string, so PREPEND will be similar to SET in this special case.

### Return value
[Integer reply](https://redis.io/topics/protocol#integer-reply): the length of the string after the prepend operation.


















# Remark 
Based on https://github.com/RedisLabsModules/redex
