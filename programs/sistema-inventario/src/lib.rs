use anchor_lang::prelude::*;

declare_id!("7bwu771KBABE6LXXuzNVNgPyRffn8VLFC6mw8s1mfCsj");

#[program]
pub mod sistema_inventario {
    use super::*;

    pub fn inicializar_inventario(ctx: Context<Inicializar>) -> Result<()> {
        let estado = &mut ctx.accounts.estado_global;
        estado.total_ventas = 0;
        estado.admin = ctx.accounts.admin.key();
        Ok(())
    }

    pub fn crear_producto(ctx: Context<CrearProducto>, nombre: String, precio: u64, stock: u32) -> Result<()> {
        let producto = &mut ctx.accounts.producto;
        producto.nombre = nombre;
        producto.precio = precio;
        producto.stock = stock;
        Ok(())
    }

    pub fn vender_producto(ctx: Context<VenderProducto>, cantidad: u32) -> Result<()> {
        let producto = &mut ctx.accounts.producto;
        let estado = &mut ctx.accounts.estado_global;
        require!(producto.stock >= cantidad, ErrorCode::StockInsuficiente);
        producto.stock -= cantidad;
        estado.total_ventas += producto.precio.checked_mul(cantidad as u64).unwrap();
        Ok(())
    }

    pub fn editar_producto(ctx: Context<EditarProducto>, nuevo_precio: Option<u64>, nuevo_stock: Option<u32>) -> Result<()> {
        let producto = &mut ctx.accounts.producto;
        if let Some(p) = nuevo_precio { producto.precio = p; }
        if let Some(s) = nuevo_stock { producto.stock = s; }
        Ok(())
    }

    pub fn eliminar_producto(_ctx: Context<EliminarProducto>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Inicializar<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8, seeds = [b"global"], bump)]
    pub estado_global: Account<'info, EstadoGlobal>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(nombre: String)]
pub struct CrearProducto<'info> {
    #[account(init, payer = usuario, space = 8 + 32 + 8 + 4 + 32, seeds = [b"prod", nombre.as_bytes()], bump)]
    pub producto: Account<'info, Producto>,
    #[account(mut)]
    pub usuario: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VenderProducto<'info> {
    #[account(mut)]
    pub producto: Account<'info, Producto>,
    #[account(mut, seeds = [b"global"], bump)]
    pub estado_global: Account<'info, EstadoGlobal>,
    pub usuario: Signer<'info>,
}

#[derive(Accounts)]
pub struct EditarProducto<'info> {
    #[account(mut)]
    pub producto: Account<'info, Producto>,
    pub usuario: Signer<'info>,
}

#[derive(Accounts)]
pub struct EliminarProducto<'info> {
    #[account(mut, close = usuario, seeds = [b"prod", producto.nombre.as_bytes()], bump)]
    pub producto: Account<'info, Producto>,
    #[account(mut)]
    pub usuario: Signer<'info>,
}

#[account]
pub struct EstadoGlobal {
    pub admin: Pubkey,
    pub total_ventas: u64,
}

#[account]
pub struct Producto {
    pub nombre: String,
    pub precio: u64,
    pub stock: u32,
}

#[error_code]
pub enum ErrorCode {
    #[msg("No hay stock suficiente.")]
    StockInsuficiente,
}