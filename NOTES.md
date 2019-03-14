# Notes

## Ideas
- BitBoardVec or Stack
  - Tak requires a 3rd dimension. A stack *should* be enough to represent this
  - Can push/pop/insert/remove at
  - Can intersect/union which collapses to a single bitboard ( multi-threaded? )
  - Can shift all bitboards in the stack at once ( multi-threaded? )


- Performance
  - Could we make shift functions avoid clones? Maybe we create specialized versions that copy the data pre-shift to the stack, and apply it after? i.e
    - For each block
      - Copy block to temp
      - Perform shift in-place
      - block |= temp
    - This would avoid needing to clone for every shift. Maybe we do this with a function that takes a list of shifts you want to do?

- Movement
  - Create functional programmy movement api, could look something like this in the end:

```
let mut bb = BitBoard<U8>::default();
let queen_moves = bb.moves()
            .left(1).repeat(8)
            .right(1).repeat(8)
            .up(1).repeat(8)
            .down(1).repeat(8)
            .collect();

let knight_moves = bb.move()
                  .up(1).left(2)
                  .up(1).right(2)
                  .up(2).left(1)
                  .up(2).right(1)
                  .mirror().collect();
```
  
  - `left/right/up/down(n)` - 
  - `repeat(n)` - repeats the last move n times
  - `mirror()` - does the opposite of all preceding moves and appends
  - `collect()` - returns a bitboard of all the moves applied to the original bitboard - non-destructively

## TODOs

- Tests!!
- Benchmarks
- Clear up the boards' orientation - would be nice ( but not overly necessary ) if left shifts actually moved left
- Generate a last_block_mask ( should be possible to do this statically )
- Generate a left/right board mask ( this will have to be dynamic I think, unless there's some sweet maths I don't know about )
- Double check we can't do a tuple at line 156, two matches suck
- Consider creating move_up/left/right functions instead of direct shifts...
- Consider the trade-offs for heap vs stack allocated bit-boards. Not too late to limit the size to 8x8 and just have a u64
- Error handling
  -  is_set should return a Result
  -  map_coords should return a Result/Option
-  Internal threading
   -  Do we want threaded shifts for bitboards with lots of blocks?

