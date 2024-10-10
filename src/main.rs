fn main() {
    println!("Hello, world!");
}
use anyhow::Result;
use http::Uri;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::{io, vec};
use tokio::sync::RwLock;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Gauge};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::{Borders, Paragraph, StatefulWidget, Widget},
    DefaultTerminal,
};
use std::time::Duration;

mod model;
mod ui;

struct App {
    model: Arc<RwLock<model::Nodes>>,
}
