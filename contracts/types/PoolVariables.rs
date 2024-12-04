#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PoolVariables {
    pub is_public: bool,               // Indica si el pool es público
    pub commission: u128,              // Comisión asociada al pool
    pub coef: u128,                    // Coeficiente de regresión
    pub intercept: u128,               // Intercepción de regresión
    pub n_distributions: u128,         // Número de distribuciones
    pub index: u128,                   // Índice del pool
    pub first_distribution_date: u128, // Fecha de la primera distribución
    pub prev_distribution_date: u128,  // Fecha de la distribución previa
    pub distribution_interval: u128,   // Intervalo entre distribuciones
}
