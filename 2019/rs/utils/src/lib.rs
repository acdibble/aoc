pub mod fractions;
pub mod grid;
pub(crate) mod matrices;
pub(crate) mod transformations;
pub mod tuples;

struct PermutationIterator<const N: usize, T: Copy> {
    state: [T; N],
    stack: [usize; N],
    pointer: usize,
    initial_done: bool,
}

impl<const N: usize, T: Copy> Iterator for PermutationIterator<N, T> {
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        if !self.initial_done {
            self.initial_done = true;
            return Some(self.state);
        }

        while self.pointer < N {
            if self.stack[self.pointer] < self.pointer {
                if self.pointer % 2 == 0 {
                    self.state.swap(0, self.pointer);
                } else {
                    self.state.swap(self.stack[self.pointer], self.pointer);
                }

                self.stack[self.pointer] += 1;
                self.pointer = 1;
                return Some(self.state);
            } else {
                self.stack[self.pointer] = 0;
                self.pointer += 1;
            }
        }

        None
    }
}

pub fn permute<const N: usize, T: Copy>(input: [T; N]) -> impl Iterator<Item = [T; N]> {
    PermutationIterator {
        state: input,
        stack: [0; N],
        pointer: 1,
        initial_done: false,
    }
}

pub fn char_to_i32(ch: char) -> i32 {
    match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => unreachable!(),
    }
}
