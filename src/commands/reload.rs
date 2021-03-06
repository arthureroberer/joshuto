use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::util::load_child::LoadChild;

pub fn soft_reload(index: usize, context: &mut JoshutoContext) -> std::io::Result<()> {
    let sort_option = context.config_t.sort_option.clone();
    if let Some(curr_tab) = context.tab_context_mut().tab_mut(index) {
        if let Some(curr_list) = curr_tab.curr_list_mut() {
            if curr_list.need_update() {
                curr_list.reload_contents(&sort_option)?;
            }
        }
        if let Some(curr_list) = curr_tab.parent_list_mut() {
            if curr_list.need_update() {
                curr_list.reload_contents(&sort_option)?;
            }
        }
        if let Some(curr_list) = curr_tab.child_list_mut() {
            if curr_list.need_update() {
                curr_list.reload_contents(&sort_option)?;
            }
        }
    }
    Ok(())
}

pub fn reload(context: &mut JoshutoContext, index: usize) -> std::io::Result<()> {
    let sort_option = context.config_t.sort_option.clone();
    if let Some(curr_tab) = context.tab_context_mut().tab_mut(index) {
        if let Some(curr_list) = curr_tab.curr_list_mut() {
            curr_list.reload_contents(&sort_option)?;
        }
        if let Some(curr_list) = curr_tab.parent_list_mut() {
            curr_list.reload_contents(&sort_option)?;
        }
        if let Some(curr_list) = curr_tab.child_list_mut() {
            curr_list.reload_contents(&sort_option)?;
        }
    }
    Ok(())
}

pub fn reload_dirlist(context: &mut JoshutoContext) -> JoshutoResult<()> {
    reload(context, context.tab_context_ref().get_index())?;
    LoadChild::load_child(context)?;
    Ok(())
}
