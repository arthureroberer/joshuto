use std::fs;
use std::path;

use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::config::keymap;
use crate::context::JoshutoContext;
use crate::error::JoshutoError;
use crate::ui;
use crate::window::JoshutoView;

#[derive(Clone, Debug)]
pub struct DeleteFiles;

impl DeleteFiles {
    pub fn new() -> Self {
        DeleteFiles
    }
    pub const fn command() -> &'static str {
        "delete_files"
    }

    pub fn remove_files(paths: Vec<path::PathBuf>) -> Result<(), std::io::Error> {
        for path in &paths {
            if let Ok(metadata) = fs::symlink_metadata(path) {
                if metadata.is_dir() {
                    fs::remove_dir_all(&path)?;
                } else {
                    fs::remove_file(&path)?;
                }
            }
        }
        Ok(())
    }

    fn delete_files(
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), std::io::Error> {
        ui::wprint_msg(&view.bot_win, "Delete selected files? (Y/n)");
        ncurses::timeout(-1);
        ncurses::doupdate();

        let curr_tab = &mut context.tabs[context.curr_tab_index];
        let mut ch = ncurses::getch();
        if ch == 'y' as i32 || ch == keymap::ENTER as i32 {
            if let Some(paths) = curr_tab.curr_list.get_selected_paths() {
                if paths.len() > 1 {
                    ui::wprint_msg(&view.bot_win, "Are you sure? (y/N)");
                    ncurses::doupdate();
                    ch = ncurses::getch();
                } else {
                    ch = 'y' as i32;
                }
                if ch == 'y' as i32 {
                    Self::remove_files(paths)?;
                    ui::wprint_msg(&view.bot_win, "Deleted files");

                    curr_tab.reload_contents(&context.config_t.sort_option)?;

                    if let Some(s) = curr_tab.curr_list.index {
                        curr_tab.curr_list.pagestate.update_page_state(
                            s,
                            view.mid_win.rows,
                            curr_tab.curr_list.contents.len(),
                            context.config_t.scroll_offset,
                        );
                    }
                    curr_tab.refresh_curr(&view.mid_win);
                    curr_tab.refresh_parent(&view.left_win, &context.config_t);
                    curr_tab.refresh_preview(&view.right_win, &context.config_t);
                }
            }
        }
        curr_tab.refresh_file_status(&view.bot_win);
        curr_tab.refresh_path_status(&view.top_win, context.config_t.tilde_in_titlebar);
        ncurses::doupdate();
        Ok(())
    }
}

impl JoshutoCommand for DeleteFiles {}

impl std::fmt::Display for DeleteFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for DeleteFiles {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        match Self::delete_files(context, view) {
            Ok(_) => Ok(()),
            Err(e) => Err(JoshutoError::IO(e)),
        }
    }
}
