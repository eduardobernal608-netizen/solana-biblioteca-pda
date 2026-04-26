# Entrega Proyecto Final Solana - Registro de Mascotas

Este es mi proyecto para la certificación. Decidí hacer un sistema de registro de mascotas porque quería aplicar lo aprendido en un caso distinto al de la biblioteca que vimos en clase.

## Lo que hace el programa:
El contrato permite gestionar un registro básico de mascotas en la red de Solana (Devnet). Cumple con todas las operaciones requeridas:
- **Crear:** Registras a la mascota con su nombre y especie.
- **Actualizar:** Tiene una función para subirle la edad.
- **Borrar:** Se puede eliminar el registro para cerrar la cuenta y recuperar el depósito de SOL (renta).

## Notas Técnicas:
- Usé **PDAs** para que cada registro sea único y esté amarrado a la wallet de quien lo crea. La semilla que usé es `"mascota"`.
- El código está hecho con **Anchor** y lo trabajé en **Solana Playground**.
- Incluí validaciones para que solo el dueño de la mascota pueda editar o borrar sus datos (usando `constraint`).

Hice el `build` en Playground y compiló sin errores. No pude hacer el deploy final por falta de SOL en la devnet (el faucet fallaba), pero el código en `lib.rs` está listo y verificado.
