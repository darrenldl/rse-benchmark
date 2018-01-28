## 2.3.2
  - Made `==`(`eq`) for `ReedSolomon` more reasonable
    - Previously `==` would compare
      - data shard count
      - parity shard count
      - total shard count
      - internal encoding matrix
      - internal `ParallelParam`
    - Now it only compares
      - data shard count
      - parity shard count

## 2.3.1
  - Added info on encoding behaviour to doc

## 2.3.0
  - Made Reed-Solomon codec creation methods return error instead of panic when shard numbers are not correct

## 2.2.0
  - Fixed SBS error checking code
  - Documentation fixes and polishing
  - Renamed `Error::InvalidShardsIndicator` to `Error::InvalidShardFlags`
  - Added more details to documentation on error handling
  - Error handling code overhaul and checks for all method variants
  - Dead commented out code cleanup and indent fix

## 2.1.0
  - Added Nicolas's SIMD C code files, gaining major speedup on supported CPUs
  - Added support for "shard by shard" encoding, allowing easier streamed encoding
  - Added functions for shard by shard encoding

## 2.0.0
  - Complete rewrite of most code following Klaus Post's design
  - Added optimsations(parallelism, loop unrolling)
  - 4-5x faster than `1.X.X`

## 1.1.1
  - Documentation polish
  - Added documentation badge to README
  - Optimised internal matrix related operations
    - This largely means `decode_missing` is faster

## 1.1.0
  - Added more helper functions
  - Added more tests
 
## 1.0.1
  - Added more tests
  - Fixed decode_missing
    - Previously may reconstruct the missing shards with incorrect length

## 1.0.0
  - Added more tests
  - Added integration with Codecov (via kcov)
  - Code refactoring
  - Added integration with Coveralls (via kcov)

## 0.9.1
  - Code restructuring
  - Added documentation

## 0.9.0
  - Base version
