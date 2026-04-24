use anchor_lang::prelude::*;

declare_id!("3qSbmtvKTwKC9gTbjFUe9XjkjYo3EeXmrPBgLqYzWRLs");

#[program]
pub mod task_system {
    use super::*;

    // CREATE: Crear una nueva tarea
    pub fn create_task(ctx: Context<CreateTask>, title: String, description: String) -> Result<()> {
        let task = &mut ctx.accounts.task_account;
        task.author = *ctx.accounts.author.key;
        task.title = title;
        task.description = description;
        task.is_done = false;
        msg!("Tarea creada exitosamente!");
        Ok(())
    }

    // UPDATE: Marcar como completada o editar
    pub fn update_task(ctx: Context<UpdateTask>, is_done: bool) -> Result<()> {
        let task = &mut ctx.accounts.task_account;
        task.is_done = is_done;
        msg!("Estado de la tarea actualizado!");
        Ok(())
    }

    // DELETE: Eliminar la tarea y recuperar los SOL
    pub fn delete_task(_ctx: Context<DeleteTask>) -> Result<()> {
        msg!("Tarea eliminada y fondos recuperados.");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateTask<'info> {
    // Definimos la PDA usando el título como semilla (seed)
    #[account(
        init, 
        payer = author, 
        space = 8 + 32 + 4 + 40 + 4 + 100 + 1, 
        seeds = [b"task", author.key().as_ref(), title.as_bytes()], 
        bump
    )]
    pub task_account: Account<'info, Task>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTask<'info> {
    #[account(mut, has_one = author)] // Seguridad: solo el autor puede editar
    pub task_account: Account<'info, Task>,
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteTask<'info> {
    // El atributo 'close' elimina la cuenta y envía los SOL al autor
    #[account(mut, has_one = author, close = author)] 
    pub task_account: Account<'info, Task>,
    #[account(mut)]
    pub author: Signer<'info>,
}

#[account]
pub struct Task {
    pub author: Pubkey,    // 32 bytes
    pub title: String,     // 4 + 40 bytes
    pub description: String, // 4 + 100 bytes
    pub is_done: bool,     // 1 byte
}
