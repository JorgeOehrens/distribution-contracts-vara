/// Enumerador para representar los tipos de almacenamiento
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageType {
    Uint256Type,   // Tipo uint256
    AddressType,   // Tipo dirección
    StringType,    // Tipo cadena de texto
    BytesType,     // Tipo bytes
    BoolType,      // Tipo booleano
    Int256Type,    // Tipo int256
    Bytes32Type,   // Tipo bytes32
}
