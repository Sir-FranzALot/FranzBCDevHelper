use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn create_docker_container(state: State<'_, AppState>) -> Result<(), String> {
    todo!()
}
