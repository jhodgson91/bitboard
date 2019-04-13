# Notes

## Ideas
- BitBoardVec or Stack
  - Tak requires a 3rd dimension. A stack *should* be enough to represent this
  - Can push/pop/insert/remove at
  - Can intersect/union which collapses to a single bitboard ( multi-threaded? )
  - Can shift all bitboards in the stack at once ( multi-threaded? )

## TODOs

- Tests!!
- Benchmarks
  - When rust gets [const generics](https://github.com/rust-lang/rust/issues/44580), we can get the best of both worlds. Apparently they're aiming for that this year.
-  Internal threading 
   -  WASM actually doesn't support threading yet, but maybe we turn threading on for non WASM builds?
   -  Do we want threaded shifts for bitboards with lots of blocks?

