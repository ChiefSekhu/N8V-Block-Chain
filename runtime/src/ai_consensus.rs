use ai_optimizer::SmartContractOptimizer;

pub struct N8IVSmartContract {
    pub code: Vec<u8>,
}

impl N8IVSmartContract {
    pub fn optimize(&mut self) -> Result<(), &'static str> {
        let optimized_code = SmartContractOptimizer::optimize(&self.code)?;
        self.code = optimized_code;
        Ok(())
    }
}
