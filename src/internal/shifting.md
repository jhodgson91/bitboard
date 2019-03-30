# Shifting contiguous memory
I need to write this out because there's a lot going on.

Shifting contiguous memory blocks requires saving the lost bits from each shift, and applying them to the next block. For BitBoards, there are some extra steps we need in between, depending on which shift you are performing.

For NxN bitboards, we have the possibility of multiple R sized blocks. ( e.g a 5x5 bitboard with 16-bit blocks requires 2 blocks ). To achieve shifting, we need to ensure that any lost bits before the shift are applied to the next block in the shift.

The next block in the shift depends on Left/Right shifting. Left shifting will traverse blocks from start to end, while right must be in reverse.

## Up/Down movement
Up/Down is the simple case. Here, we just need to perform shifts without worrying about masking out edges. The algorithm should do the following

- While RHS > 0
  - For each block
    - Reverse shift the block by a maximum of block size and store the result
    - Shift the block by a maximum of block size
    - Apply previous lost bits
  - Apply last block mask

## Left/Right movement
Left/Right movement is more complex, and requires edge masking based on the amount the user requested

- While RHS > 0
  - For each block
    - Reverse shift the block by a max of block size and store
    - Shift the block by a max of block size
    - Apply previous lost bits
    - Apply edge mask
  - Apply last block mask

## Last Block Mask
Last block mask must be applied at the end of shifting to ensure that no bits are left in the gap between the end of the board and the end of the last block.
```
last_block_mask<R: PrimUInt>(board_width) -> R {
    let remainder = Self::board_size() % (mem::size_of<R>() * 8)
    match remainder {
        0 => R::zero(),
        _ => (R::one() + R::one()).pow(remainder as u32) - R::one(),
    }
}
```

## Edge mask
Edge masking is only used for Left/Right movement. If somebody requests a left move of 2, then all bits on the right 2 files of the board should be zeroed out. 

If a shift move is requested that's greater than the width of the board, the whole board is zeroed.

### Options
I could think of two options here, either we
- Generate a BitBoard with the correct edge mask, and apply it at the end of shifting
- Apply edge masking during the shift on a block-by-block basis. 

Second option is way better as it avoids allocating an entirely new bitboard just for the masking. HOWEVER, as I have discovered, it's much more complicated.

Option 1 has the benefit of just applying an or to the whole board. Option 2 has issues when you shift further than a single block, since we need to track what happened to every block before and apply it after...

### Edge masking per block
To achieve edge masking per block, we need to know a fair bit:
- Direction of shift
- width of the edge mask
- index of the block we're masking
- Width of the board
- Size of the blocks

With this data, we're able to figure out which bits in a block
correspond to the edge of the board we're working on

The below is a bit of a mind fuck, but it works. It's easier to think about
with block_idx = 0. It maps block bits to the whole board and figures out which
are width away from Left/Right of the board
```
edge_mask<R: PrimUInt>(dir: Shift, mut width: usize, block_idx: usize, board_width: usize) -> R {
    if width >= board_width {
        return R::zero();
    }
    let block_bits = mem::size_of<R>() * 8;

    !(0..block_bits)
        .into_iter()
        .filter(|i| match dir {
            Shift::Right => ((block_bits * block_idx) + i) % board_width < width,
            Shift::Left => {
                board_width - (((block_bits * block_idx) + i) % board_width) - 1
                    < width
            }
        })
        .fold(R::zero(), |a, b| a | R::one() << b)
}
```

## Move enum
NB: Left shift == sends bits right and up the board ( right now )

Up/Down is easy, it will be an exact multiple of N and there's no ambiguity

Left/Right depends on the distance away from the nearest N If a user requested a left shift of 7 on an 8x8 board, we can assume they were going up 1, left 1. This breaks down at the boundary though. If a user left shifts 4, they might be trying to go right 4, or up 1 left 4.

There's an argument to be made that right 4 is, in this case, a more sensible move, but it would be nice if we could be generic. I also don't think that's the only case. There are many grey areas

The Move enum solves this problem. It forces users to specify intention, meaning we can determine the width of the edge mask more easily

# Functional Movement
Rust often uses functional features for abstraction. For example iterators

I'm attempting to make this for board moves. Eventually, it would be cool if we could do this:

```
let mut bb = BitBoard<U8>::new(vec![3,3]);
let castle_moves = bb.moves()
            .left(1).repeat(8)
            .right(1).repeat(8)
            .up(1).repeat(8)
            .down(1).repeat(8)
            .collect();

let knight_moves = bb.moves()
                  .up(1).left(2)
                  .up(1).right(2)
                  .up(2).left(1)
                  .up(2).right(1)
                  .mirror().collect();
```
There are two things I want to achieve with the functional api
- Better Performance
- Better API than just doing shifts

### Performance
Currently, the Shl implementation requires a clone because we don't want to consume the bitboard we're shifting. Every clone is a heap allocation, so for some moves we would need to do a hecking lot of allocations.

There's no way to entirely avoid this, but we can minimise it.

### Better API than plain shifts
Generating complicated piece moves would either be lots of loops or a bit ass shift. With the functional front-end, we could provide transformations like rotate and mirror to cut this back.

## Design Problems

### Collapsing Moves
We need some way of *collapsing* moves down, but only at appropriate times...

```
let mut bb = BitBoard;

// This should translate to UpLeft(1)
bb.moves().left(1).up(1).collect()

// This should be a list of Left(1..7) moves
bb.moves().left(1).repeat(8).collect()

// This should be a list of Left(1) Right(1)
bb.moves().left(1).mirror().collect()

// This should be list of Left(1..7) and Right(1..7)
bb.moves().left(1).repeat(8).mirror().collect()
```

### Collecting Moves
Do we collect as we go? Or can we create a list of moves and apply them all at the end?
I think collect as we go...