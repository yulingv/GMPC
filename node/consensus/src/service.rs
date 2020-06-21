

/// Algorithm used for proof of work.
pub trait PowAlgorithm<B: BlockT> {
    /// Difficulty for the algorithm.
    type Difficulty: TotalDifficulty + Default + Encode + Decode + Ord + Clone + Copy;

    /// Get the next block's difficulty.
    ///
    /// This function will be called twice during the import process, so the implementation
    /// should be properly cached.
    fn difficulty(&self, parent: B::Hash) -> Result<Self::Difficulty, Error<B>>;
    /// Verify that the seal is valid against given pre hash when parent block is not yet imported.
    ///
    /// None means that preliminary verify is not available for this algorithm.
    fn preliminary_verify(
        &self,
        _pre_hash: &B::Hash,
        _seal: &Seal,
    ) -> Result<Option<bool>, Error<B>> {
        Ok(None)
    }
    /// Verify that the difficulty is valid against given seal.
    fn verify(
        &self,
        parent: &BlockId<B>,
        pre_hash: &B::Hash,
        seal: &Seal,
        difficulty: Self::Difficulty,
    ) -> Result<bool, Error<B>>;
    /// Mine a seal that satisfies the given difficulty.
    fn mine(
        &self,
        parent: &BlockId<B>,
        pre_hash: &B::Hash,
        difficulty: Self::Difficulty,
        round: u32,
    ) -> Result<Option<Seal>, Error<B>>;
}