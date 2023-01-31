pub const DEFAULT_CONFIG: &str = r#"
    [theme]
    basic_fg = 'Gray'
    basic_bg = 'Reset'
    window_border = 'Blue'
    window_title = 'LightRed'
    secondary_title = 'Blue'
    active_window_title = 'Gray'
    active_border = 'Gray'
    active_title = 'Gray'

    list_highlighted_fg = 'LightGreen'
    list_highlighted_bg = 'DarkGray'
    list_active_fg = 'LightYellow'
    list_active_bg = 'DarkGray'

    list_directory_fg = 'Blue'
    #list_directory_active_fg = 'Blue'
    #list_directory_active_bg = 'DarkGray'

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
    template_popup = 't'

    # Files Screen Actions
    add_file = 's'
    add_all_files = 'a'
    parent_directory = 'b'
    enter_directory = 'Enter'
    show_hidden = 't'

    # Frames Screen Actions
    add_frame = 'a'

    # Popup Actions
    select_field = 'Enter'
    save_changes = 'w'
"#;
