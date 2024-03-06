// writing bounded proofs
fn initialize_prefix(length: usize, buffer: &mut [u8]) {
    // Let's just ignore invalid calls
    if length > buffer.len() {
        return;
    }

    for i in 0..=length {
        buffer[i] = 0;
    }
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(11)] // deliberately too low
// By putting #[kani::unwind(1)] on the proof harness, we've placed an upper bound of 1 loop iteration. The "unwinding assertion" failure that Kani reports is because this bound is not high enough. The code tries to execute more than 1 loop iteration. 
fn check_initialize_prefix() {
    const LIMIT: usize = 10;
    let mut buffer: [u8; LIMIT] = [1; LIMIT];

    let length = kani::any();
    kani::assume(length <= LIMIT);

    initialize_prefix(length, &mut buffer);
}

//Kani needs the unwinding bound to be "one more than" the number of loop iterations. We previously had an off-by-one error that tried to do 11 iterations on an array of size 10. So... the unwinding bound needed to be 11, then.
