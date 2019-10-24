[![GitHub issues](https://img.shields.io/github/release/RedisLabsModules/RedisX.svg)](https://github.com/RedisLabsModules/RedisX/releases/latest)
[![CircleCI](https://circleci.com/gh/RedisLabsModules/RedisX/tree/master.svg?style=svg)](https://circleci.com/gh/RedisLabsModules/RedisX/tree/master)

# RedisX
Extension modules to Redis' native data types and commands

# Commands

## X.GETSETEX key value seconds
`Time complexity: O(1)`

Atomically sets key to value and returns the old value stored at key
and set key to timeout after a given number of seconds.
Returns an error when key exists but does not hold a string value.

This command is equivalent to executing the following commands:

```
GETSET mykey value
EXPIRE mykey seconds
```

### Return value
[Bulk string reply](https://redis.io/topics/protocol#bulk-string-reply): the value of key, or nil when key does not exist.



## X.GETEX key seconds
`Time complexity: O(1)`

Get the value of key and set key to timeout after a given number of seconds.
If the key does not exist the special value nil is returned. 
An error is returned if the value stored at key is not a string, because GETEX only handles string values.

### Return value
[Bulk string reply](https://redis.io/topics/protocol#bulk-string-reply): the value of key, or nil when key does not exist.



## X.PREPEND key value
`Time complexity: O(1). The amortized time complexity is O(1) assuming the appended value is small and the already present value is of any size`

If key already exists and is a string, this command prepend the value at the begin of the string. 
If key does not exist it is created and set as an empty string, so PREPEND will be similar to SET in this special case.

### Return value
[Integer reply](https://redis.io/topics/protocol#integer-reply): the length of the string after the prepend operation.



# Remark 
Based on https://github.com/RedisLabsModules/redex
