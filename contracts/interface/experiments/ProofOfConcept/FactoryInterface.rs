use gstd::{msg, prelude::*, ActorId};
use hashbrown::HashMap;

#[derive(Default)]
pub struct Factory {
    logic_versions: HashMap<String, ActorId>, // Mapea versiones a direcciones lógicas
    proxies: HashMap<String, Proxy>,          // Mapea nombres de proxies a sus datos
    latest_version: Option<String>,           // Almacena la última versión añadida
}

#[derive(Debug, Clone)]
pub struct Proxy {
    name: String,
    logic_address: ActorId, // Dirección lógica actual del proxy
}

impl Factory {
    /// Añade una nueva versión lógica al Factory
    pub fn add_logic_version(&mut self, logic: ActorId, version: String) {
        self.logic_versions.insert(version.clone(), logic);
        self.latest_version = Some(version);
    }

    /// Obtiene la dirección lógica asociada a una versión específica
    pub fn get_logic_version(&self, version: String) -> Option<ActorId> {
        self.logic_versions.get(&version).cloned()
    }

    /// Obtiene la dirección lógica de la última versión
    pub fn get_latest_version(&self) -> Option<ActorId> {
        if let Some(latest) = &self.latest_version {
            self.get_logic_version(latest.clone())
        } else {
            None
        }
    }

    /// Crea un nuevo proxy con una lógica predeterminada
    pub fn create_proxy(&mut self, proxy_name: String) -> Option<ActorId> {
        if let Some(latest_logic) = self.get_latest_version() {
            let proxy = Proxy {
                name: proxy_name.clone(),
                logic_address: latest_logic,
            };
            self.proxies.insert(proxy_name.clone(), proxy);
            Some(latest_logic)
        } else {
            None // No hay lógica disponible para asignar al proxy
        }
    }

    /// Actualiza la lógica de un proxy existente a una versión específica
    pub fn update_proxy_logic(&mut self, proxy_name: String, version: String) -> Result<(), String> {
        if let Some(proxy) = self.proxies.get_mut(&proxy_name) {
            if let Some(new_logic) = self.get_logic_version(version) {
                proxy.logic_address = new_logic;
                Ok(())
            } else {
                Err("Versión lógica no encontrada".to_string())
            }
        } else {
            Err("Proxy no encontrado".to_string())
        }
    }
}
