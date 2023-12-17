mod postgres;

use crate::app_state::AppState;

pub fn on_start(app_state: &AppState) {
    postgres::on_start(app_state)
}