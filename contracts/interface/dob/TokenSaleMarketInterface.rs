use gstd::{msg, prelude::*, ActorId};
use hashbrown::HashMap;

#[derive(Default)]
pub struct TokenSaleMarket {
    sale_properties: HashMap<ActorId, SaleProperties>,
}

#[derive(Debug, Clone)]
pub struct SaleProperties {
    sale_price: u128,
    min_division: u128,
    apply_commission: bool,
    lock_status: bool,
}

impl TokenSaleMarket {
    /// Establece las propiedades de venta para un token
    pub fn set_sale_properties(
        &mut self,
        token_address: ActorId,
        sale_price: u128,
        min_division: u128,
    ) {
        let properties = SaleProperties {
            sale_price,
            min_division,
            apply_commission: false, // Default value
            lock_status: false,      // Default value
        };
        self.sale_properties.insert(token_address, properties);
    }

    /// Establece propiedades iniciales de venta para un pool
    pub fn set_initial_sale_properties(
        &mut self,
        pool_address: ActorId,
        sale_price: u128,
        min_division: u128,
    ) {
        self.set_sale_properties(pool_address, sale_price, min_division);
    }

    /// Recupera las propiedades de venta de un token
    pub fn get_sale_properties(
        &self,
        token_address: ActorId,
    ) -> Option<(u128, u128, bool, bool)> {
        self.sale_properties.get(&token_address).map(|props| {
            (
                props.sale_price,
                props.min_division,
                props.apply_commission,
                props.lock_status,
            )
        })
    }

    /// Compra tokens
    pub fn buy_token(
        &mut self,
        n_token_to_buy: u128,
        seller: ActorId,
        token_address: ActorId,
    ) {
        let buyer = msg::source();
        if let Some(properties) = self.sale_properties.get(&token_address) {
            assert!(
                !properties.lock_status,
                "La venta de este token está bloqueada"
            );

            let total_price = n_token_to_buy * properties.sale_price;
            assert!(
                msg::value() >= total_price,
                "No se envió suficiente dinero para comprar los tokens"
            );

            // Transferir tokens lógicamente (o enviar mensajes)
            msg::send(seller, total_price, 0).expect("Fallo en la transferencia de fondos");
            msg::reply(buyer, n_token_to_buy, 0).expect("Fallo al enviar tokens al comprador");
        } else {
            panic!("Token no encontrado para la venta");
        }
    }

    /// Bloquea la venta de un token
    pub fn lock_sale(&mut self, token_address: ActorId) {
        if let Some(properties) = self.sale_properties.get_mut(&token_address) {
            properties.lock_status = true;
        } else {
            panic!("Token no encontrado para bloquear la venta");
        }
    }

    /// Desbloquea la venta de un token
    pub fn unlock_sale(&mut self, token_address: ActorId) {
        if let Some(properties) = self.sale_properties.get_mut(&token_address) {
            properties.lock_status = false;
        } else {
            panic!("Token no encontrado para desbloquear la venta");
        }
    }
}
