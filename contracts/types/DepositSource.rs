/// Enum para representar las fuentes de depósito
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepositSource {
    Metamask,  // source: 0
    Platform,  // source: 1
    Internal,  // source: 2
}

impl DepositSource {
    /// Convierte un índice (u8) a una variante del enum DepositSource
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(DepositSource::Metamask),
            1 => Some(DepositSource::Platform),
            2 => Some(DepositSource::Internal),
            _ => None, // Retorna None si el índice no es válido
        }
    }

    /// Convierte una variante del enum DepositSource a su índice (u8)
    pub fn as_u8(self) -> u8 {
        match self {
            DepositSource::Metamask => 0,
            DepositSource::Platform => 1,
            DepositSource::Internal => 2,
        }
    }
}
