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
