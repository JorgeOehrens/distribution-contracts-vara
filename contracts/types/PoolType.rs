/// Enumerador para los tipos de pool
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoolType {
    Treasury, // Tipo: Tesorería
    Payroll,  // Tipo: Nómina
    Reward,   // Tipo: Recompensas
}

impl PoolType {
    /// Convierte un índice (u8) a una variante del enum PoolType
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(PoolType::Treasury),
            1 => Some(PoolType::Payroll),
            2 => Some(PoolType::Reward),
            _ => None, // Retorna None para valores no válidos
        }
    }

    /// Convierte una variante del enum PoolType a su índice (u8)
    pub fn as_u8(self) -> u8 {
        match self {
            PoolType::Treasury => 0,
            PoolType::Payroll => 1,
            PoolType::Reward => 2,
        }
    }
}
