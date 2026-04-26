use anchor_lang::prelude::*;

declare_id!("E7N74m83fyS8g2cjShwqHgykQVsRwRz6o3cAQUjjCs1N");

#[program]
pub mod registro_mascotas {
    use super::*;

    pub fn registrar_mascota(ctx: Context<RegistrarMascota>, nombre: String, especie: String) -> Result<()> {
        let mascota = &mut ctx.accounts.mascota;
        mascota.dueno = ctx.accounts.dueno.key(); // Cambiamos 'owner' por 'dueno' para evitar conflictos
        mascota.nombre = nombre;
        mascota.especie = especie;
        mascota.edad = 0;
        Ok(())
    }

    pub fn cumplir_anos(ctx: Context<ActualizarMascota>) -> Result<()> {
        let mascota = &mut ctx.accounts.mascota;
        mascota.edad += 1;
        Ok(())
    }

    pub fn eliminar_registro(_ctx: Context<EliminarMascota>) -> Result<()> {
        Ok(())
    }
}

#[account]
pub struct Mascota {
    pub dueno: Pubkey,   // Cambiado a 'dueno' para ser consistente
    pub nombre: String,
    pub especie: String,
    pub edad: u8,
}

#[derive(Accounts)]
pub struct RegistrarMascota<'info> {
    #[account(
        init, 
        payer = dueno, 
        space = 8 + 32 + 40 + 40 + 1, 
        seeds = [b"mascota", dueno.key().as_ref()], 
        bump
    )]
    pub mascota: Account<'info, Mascota>,
    #[account(mut)]
    pub dueno: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarMascota<'info> {
    #[account(
        mut, 
        seeds = [b"mascota", dueno.key().as_ref()], 
        bump,
        constraint = mascota.dueno == dueno.key() // Validación manual más robusta
    )]
    pub mascota: Account<'info, Mascota>,
    pub dueno: Signer<'info>,
}

#[derive(Accounts)]
pub struct EliminarMascota<'info> {
    #[account(
        mut, 
        close = dueno, 
        seeds = [b"mascota", dueno.key().as_ref()], 
        bump,
        constraint = mascota.dueno == dueno.key()
    )]
    pub mascota: Account<'info, Mascota>,
    #[account(mut)]
    pub dueno: Signer<'info>,
}
