# Distribution Contracts on Vara Network

Este repositorio contiene contratos inteligentes para la distribución de tokens en la red **Vara** usando **Gear Protocol**.

## 🚀 Requisitos

- Rust + Nightly
- Cargo
- Cuenta en [Supabase](https://supabase.com/)
- Acceso a [Gear Idea](https://idea.gear-tech.io/)

---

## 🧱 Instalación

1. Clonar este repositorio:

```bash
git clone https://github.com/JorgeOehrens/distribution-contracts-vara.git
cd distribution-contracts-vara
```

2. Ir a la carpeta del contrato que deseas compilar:

```bash
cd contracts/<nombre_contrato>
cargo +nightly build --release
```

---

## ⚙️ Despliegue en Gear Idea

1. Ir a [Gear Idea](https://idea.gear-tech.io/)

2. Crear y desplegar los siguientes contratos desde la sección `Code`:

- **Pool**
- **VFT**

3. Agregar los campos del **constructor** correspondientes a cada contrato al momento del despliegue.

---

## 🏭 Agregar contrato Factory

1. Desplegar el contrato `Factory` desde Gear Idea.

2. En el constructor, se deben pasar los siguientes argumentos:

```text
pool_id  // ID del código del contrato Pool
vft_id   // ID del código del contrato VFT
```

---

## 🛢️ Creación de Base de Datos en Supabase

1. Crear una cuenta en [Supabase](https://supabase.com/)

2. Ejecutar la siguiente query SQL desde el panel de Supabase:

```sql
CREATE TABLE pools (
    id SERIAL PRIMARY KEY,
    id_vara TEXT,
    nombre TEXT,
    modo_distribucion TEXT,
    acceso TEXT,
    tipo TEXT,
    creador TEXT,
    participantes JSONB,
    transacciones JSONB,
    created_at TIMESTAMP,
    id_token INTEGER
);

CREATE TABLE tokens (
    id SERIAL PRIMARY KEY,
    name TEXT,
    symbol TEXT,
    decimal INTEGER,
    created_at TIMESTAMPTZ,
    owner TEXT,
    txHash TEXT,
    programId TEXT,
    shares JSONB
);
```

---
