#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    CurrentAmount,      // dtype: 0, ethPool
    CurrentDistAmount,  // dtype: 1, ethPool NOT USED ANYMORE
    PrePay,             // dtype: 2, ethPool, tokenPool
    GasCost,            // dtype: 3, ethPool, tokenPool
    Distribute,         // dtype: 4, tokenPool
    UserDistribute,     // dtype: 5, ethPool, tokenPool
}

impl DataType {
    /// Convert enum variant to its index (as u8)
    pub fn as_u8(self) -> u8 {
        match self {
            DataType::CurrentAmount => 0,
            DataType::CurrentDistAmount => 1,
            DataType::PrePay => 2,
            DataType::GasCost => 3,
            DataType::Distribute => 4,
            DataType::UserDistribute => 5,
        }
    }

    /// Convert an index (u8) to a DataType variant
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(DataType::CurrentAmount),
            1 => Some(DataType::CurrentDistAmount),
            2 => Some(DataType::PrePay),
            3 => Some(DataType::GasCost),
            4 => Some(DataType::Distribute),
            5 => Some(DataType::UserDistribute),
            _ => None, // Return None for invalid values
        }
    }
}
