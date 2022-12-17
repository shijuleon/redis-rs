# redis-rs

## Why?
I'm rewriting parts of [newsfeed](https://github.com/shijuleon/newsfeed) to use Redis with [redis-rs](https://github.com/redis-rs/redis-rs).
There's a method which sends approximately a 1000 GET commands to redis sequentially but in working it was unusually slow (30s).

```redis ops took 30288 ms```

The same method takes 0.12s with this library (even with a new connection every call, multiple copying..). This library is a precursor to debugging.

```redis ops took 128 ms```

Currently this only supports two commands; `GET` and `ZREVRANGEBYSCORE`.