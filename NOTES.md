# Notes

## Ideas
- BitBoardVec or Stack
  - Tak requires a 3rd dimension. A stack *should* be enough to represent this
  - Can push/pop/insert/remove at
  - Can intersect/union which collapses to a single bitboard ( multi-threaded? )
  - Can shift all bitboards in the stack at once ( multi-threaded? )
- Block iterators
  - for_each_block and for_each_block_mut functions that take closures - could tidy up all the loops we need to do

## TODOs

- Tests!!
- Benchmarks
- Clear up the boards' orientation - would be nice ( but not overly necessary ) if left shifts actually moved left
- Generate a last_block_mask ( should be possible to do this statically )
- Generate a left/right board mas ( this will have to be dynamic I think, unless there's some sweet maths I don't know about )
- Double check we can't do a tuple at line 156, two matches suck
- Consider creating move_up/left/right functions instead of direct shifts...
- Consider the trade-offs for heap vs stack allocated bit-boards. Not too late to limit the size to 8x8 and just have a u64
- Error handling
  -  is_set should return a Result
  -  map_coords should return a Result/Option
-  Internal threading
   -  Do we want threaded shifts for bitboards with lots of blocks?

