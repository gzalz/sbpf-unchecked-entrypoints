# SBPF Unchecked Entrypoints
Define entrypoint paths for different instruction types, optionally bypassing standard entrypoint deserialization and checks for optimized instruction processing. 

This crate uses pinocchio's standard entrypoint as the underlying fallback checked entrypoint to retain developer ergonomics for unoptimized instructions.

## Macros

```rust
// Invoked unchecked_handler if is_unchecked evaluates to true, otherwise checked_entrypoint
// checked_handler:   fn (&Address, &[AccountView], &[u8])
// is_unchecked:      fn *mut u8 -> bool
// unchecked_handler: fn *mut u8 -> u64

conditional_entrypoint!(checked_handler, is_unchecked, unchecked_handler);

// Lowest level entrypoint available in this crate
// unchecked_handler: fn *mut u8 -> u64 

unchecked_entrypoint!(entrypoint);
```

## CU Cost
| Program Type | Compute Units Consumed |
|----------|----------|
| pinocchio no-op  | 31  |
| sbpf-conditional-entrypoint no-op  | 4  |
| sbpf-unchecked-entrypoint no-op  | 2  |
| asm no-op| 1 |


## Usage
Demonstration of a theoretical optimized oracle program.
```rust
// Memory layout assumes signer is 0 size system account
// and that the oracle PDA is of size 8
// See: https://sbpf.xyz/
const IXN_DATA_OFFSET: isize = 0x50d8;
const SIGNER_KEY_OFFSET: isize = 0x0010;
const ORACLE_DATA_OFFSET: isize = 0x28c0;
const MAX_TX_ACCOUNTS: usize = (u8::MAX - 1) as usize;
const AUTHORITY_PUBKEY_DW: [u64; 4] = [42, 42, 42, 42];

use sbpf_unchecked_entrypoint::conditional_entrypoint;

conditional_entrypoint!(checked_entrypoint, is_oracle_update, unchecked_entrypoint);

pub fn is_oracle_update(input: *mut u8) -> bool {
    let num_accounts = unsafe { *(input as *mut u64) };
    num_accounts == 2
}

pub unsafe fn checked_entrypoint(
    program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    // unoptimized instructions are processed here
    Ok(())
}

#[inline(always)]
pub unsafe fn unchecked_entrypoint(input: *mut u8) -> u64 {
    for i in 0..4 {
        let signer_key_dw =
            unsafe { *(input.offset(SIGNER_KEY_OFFSET + (i as isize * 8)) as *mut u64) };
        if signer_key_dw != AUTHORITY_PUBKEY_DW[i] {
            return 2 as u64;
        }
    }
    let discr = unsafe { *(input.offset(IXN_DATA_OFFSET) as *mut u64) };
    unsafe { *(input.offset(ORACLE_DATA_OFFSET) as *mut u64) = discr };
    return 0;
}
```

