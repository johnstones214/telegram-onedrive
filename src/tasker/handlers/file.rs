/*
:project: telegram-onedrive
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use super::{tasks, transfer::multi_parts_uploader_from_tg_file, Progress};
use crate::{
    client::utils::chat_from_hex,
    error::{Error, Result, TaskAbortError},
    state::AppState,
};
use proc_macros::{add_context, add_trace};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

#[add_context]
#[add_trace]
pub async fn handler(
    task: tasks::Model,
    progress: Arc<Progress>,
    cancellation_token: CancellationToken,
    state: AppState,
) -> Result<()> {
    let aborters = state.task_session.aborters.clone();

    let filename =
        match multi_parts_uploader_from_tg_file(&task, progress.clone(), cancellation_token, state)
            .await
        {
            Ok(filename) => filename,
            Err(e) => {
                if let Some(boxed_e) = e.get_raw() {
                    if boxed_e.downcast_ref::<TaskAbortError>().is_some() {
                        return Ok(());
                    }
                }
                return Err(e);
            }
        };

    let chat = chat_from_hex(&task.chat_user_hex)?;
    aborters
        .write()
        .await
        .remove(&(chat.id, task.message_id))
        .ok_or_else(|| Error::new("task aborter not found"))?;

    progress.update_filename(task.id, &filename).await?;

    Ok(())
}
