pub const DEFAULT_CONFIG: &str = r#"
    [theme]
    list_item_fg = 'LightGreen'
    list_item_bg = 'Reset'
    active_list_item_fg = 'LightYellow'
    active_list_item_bg = 'DarkGray'
    inactive_list_item_fg = 'LightGreen'
    inactive_list_item_bg = 'DarkGray'

    help_border = 'LightYellow'

    log_error_fg = 'Red'
    log_info_fg = 'Blue'
    log_trace_fg = 'DarkGray'
    log_warn_fg = 'Yellow'

    [actions]
    # General Actions
    up = 'Up'
    down = 'Down'
    quit = 'q'
    back = 'Esc'
    switch_focus = 'Tab'
    toggle_logs = 'l'
    logs_prev = 'PageUp'
    logs_next = 'PageDown'
    screen_one = '1'
    screen_two = '2'
    screen_three = '3'
    help = 'h'

    # Main Screen Actions
    remove_files = 'c'
    write_tags = 'w'
    select_current = 's'
    select_all = 'a'
    remove = 'd'
    spawn_popup = 'Enter'
    update_names = 'u'

    # Files Screen Actions
    add_file = 's'
    add_all_files = 'a'
    parent_directory = 'b'
    enter_directory = 'Enter'

    # Frames Screen Actions
    add_frame = 'a'

    # Popup Actions
    select_field = 'Enter'
    save_changes = 'w'
"#;
