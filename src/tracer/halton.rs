// ====================================================================================================================
// 1-dimensional Halton sequence
// ====================================================================================================================

///
/// Halton sequence of base N
pub struct HaltonSeq {
    base: usize,
    offset: usize
}



///
/// Methods
impl HaltonSeq {
    ///
    /// Create a new sequence with a given base
    pub fn new(base: usize) -> Self{
        HaltonSeq {
            base: base, 
            offset: 0
        }
    }


    ///
    /// Discard the next `num` elements of the sequence
    pub fn discard(&mut self, num: usize) {
        self.offset += num;
    }


    ///
    /// Generate the next element of the sequence
    pub fn next(&mut self) -> f64 {
        let mut sample = 0.0;
        let mut denominator = self.base as f64;
        let mut n = self.offset;
        while n > 0 {
            let multiplier : usize = n % self.base;
            sample += (multiplier as f64) / denominator;
            n = n / self.base;
            denominator *= self.base as f64;
        }
        self.offset += 1;

        sample
    }
}
