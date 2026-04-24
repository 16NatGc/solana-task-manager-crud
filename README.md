# 💡 solana-task-manager-crud
Backend de un sistema de gestión de tareas (To-Do List) desarrollado en Rust con Anchor, implementando CRUD y PDAs para Solana Latam.
Solana Task System (CRUD + PDA)
Este proyecto es un sistema de gestión de tareas descentralizado desarrollado para el Solana LATAM Builders Program (Abril 2026). El programa permite a los usuarios crear, actualizar y eliminar tareas directamente en la blockchain de Solana.

## Detalles del Despliegue
Program ID: 3qSbmvtKTwKC9gTbjFUe9XjkjYo3E... (Pega aquí tu ID completo)

Red: Solana Devnet

Framework: Anchor v0.29.0 (Rust)

## Funcionalidades del Contrato (CRUD)
El programa implementa las operaciones básicas de persistencia de datos:

Create (Crear): La instrucción create_task inicializa una nueva cuenta de tarea.

Read (Leer): Las tareas están almacenadas en cuentas públicas accesibles mediante el ID del programa.

Update (Actualizar): La instrucción update_task permite modificar el estado de la tarea (completada/pendiente).

Delete (Eliminar): La instrucción delete_task cierra la cuenta de la tarea y devuelve los lamports (SOL) de la renta al autor.

Uso de PDAs (Program Derived Addresses)
Para cumplir con los requisitos de la certificación, se implementaron PDAs para organizar las cuentas de forma determinista.

Semillas (Seeds): [b"task", author_pubkey, task_title]

Propósito: Esto garantiza que cada tarea sea única para el usuario y que solo el creador tenga autoridad para modificarla o cerrarla.

## Estructura del Repositorio
/src/lib.rs: Código fuente del programa en Rust.

Anchor.toml: Archivo de configuración del proyecto.

IDL.json: Interfaz de lógica del programa para facilitar la interacción.

## Pruebas y Verificación
El programa ha sido desplegado exitosamente utilizando Solana Playground. La lógica fue verificada manualmente mediante la pestaña de interacción (Test UI) de la plataforma, confirmando la creación y cierre de cuentas en la Devnet.
