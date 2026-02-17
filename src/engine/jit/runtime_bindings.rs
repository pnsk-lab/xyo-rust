use super::*;

impl<'ctx> RuntimeFunctions<'ctx> {
    pub(super) fn declare(
        context: &'ctx LlvmContext,
        module: &Module<'ctx>,
        execution_engine: &ExecutionEngine<'ctx>,
    ) -> Self {
        let void_type = context.void_type();
        let bool_type = context.bool_type();
        let f64_type = context.f64_type();
        let i64_type = context.i64_type();
        let ptr_type = context.ptr_type(AddressSpace::default());

        let count_executed_block = module.add_function(
            "rt_count_executed_block",
            void_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &count_executed_block,
            rt_count_executed_block as *const () as *const () as usize,
        );

        let move_steps = module.add_function(
            "rt_motion_move_steps",
            void_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &move_steps,
            rt_motion_move_steps as *const () as *const () as usize,
        );

        let set_direction = module.add_function(
            "rt_motion_set_direction",
            void_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &set_direction,
            rt_motion_set_direction as *const () as *const () as usize,
        );

        let change_x = module.add_function(
            "rt_motion_change_x",
            void_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &change_x,
            rt_motion_change_x as *const () as *const () as usize,
        );

        let change_y = module.add_function(
            "rt_motion_change_y",
            void_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &change_y,
            rt_motion_change_y as *const () as *const () as usize,
        );

        let set_x = module.add_function(
            "rt_motion_set_x",
            void_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine
            .add_global_mapping(&set_x, rt_motion_set_x as *const () as *const () as usize);

        let set_y = module.add_function(
            "rt_motion_set_y",
            void_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine
            .add_global_mapping(&set_y, rt_motion_set_y as *const () as *const () as usize);

        let goto_xy = module.add_function(
            "rt_motion_goto_xy",
            void_type.fn_type(&[ptr_type.into(), f64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &goto_xy,
            rt_motion_goto_xy as *const () as *const () as usize,
        );

        let motion_x_position = module.add_function(
            "rt_motion_x_position",
            f64_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &motion_x_position,
            rt_motion_x_position as *const () as *const () as usize,
        );

        let motion_y_position = module.add_function(
            "rt_motion_y_position",
            f64_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &motion_y_position,
            rt_motion_y_position as *const () as *const () as usize,
        );

        let get_var = module.add_function(
            "rt_get_var",
            f64_type.fn_type(&[ptr_type.into(), i64_type.into()], false),
            None,
        );
        execution_engine
            .add_global_mapping(&get_var, rt_get_var as *const () as *const () as usize);

        let set_var = module.add_function(
            "rt_set_var",
            void_type.fn_type(&[ptr_type.into(), i64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine
            .add_global_mapping(&set_var, rt_set_var as *const () as *const () as usize);

        let change_var = module.add_function(
            "rt_change_var",
            void_type.fn_type(&[ptr_type.into(), i64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &change_var,
            rt_change_var as *const () as *const () as usize,
        );

        let get_variables_ptr = module.add_function(
            "rt_get_variables_ptr",
            ptr_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &get_variables_ptr,
            rt_get_variables_ptr as *const () as *const () as usize,
        );

        let data_add_to_list = module.add_function(
            "rt_data_add_to_list",
            void_type.fn_type(&[ptr_type.into(), i64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &data_add_to_list,
            rt_data_add_to_list as *const () as *const () as usize,
        );

        let data_delete_of_list = module.add_function(
            "rt_data_delete_of_list",
            void_type.fn_type(&[ptr_type.into(), i64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &data_delete_of_list,
            rt_data_delete_of_list as *const () as usize,
        );

        let data_delete_all_of_list = module.add_function(
            "rt_data_delete_all_of_list",
            void_type.fn_type(&[ptr_type.into(), i64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &data_delete_all_of_list,
            rt_data_delete_all_of_list as *const () as usize,
        );

        let data_replace_item_of_list = module.add_function(
            "rt_data_replace_item_of_list",
            void_type.fn_type(
                &[
                    ptr_type.into(),
                    i64_type.into(),
                    f64_type.into(),
                    f64_type.into(),
                ],
                false,
            ),
            None,
        );
        execution_engine.add_global_mapping(
            &data_replace_item_of_list,
            rt_data_replace_item_of_list as *const () as usize,
        );

        let data_list_contains_item = module.add_function(
            "rt_data_list_contains_item",
            f64_type.fn_type(&[ptr_type.into(), i64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &data_list_contains_item,
            rt_data_list_contains_item as *const () as usize,
        );

        let say_number = module.add_function(
            "rt_looks_say_number",
            void_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&say_number, rt_looks_say_number as *const () as usize);

        let say_text = module.add_function(
            "rt_looks_say_text",
            void_type.fn_type(&[ptr_type.into(), ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&say_text, rt_looks_say_text as *const () as usize);

        let looks_switch_costume_to = module.add_function(
            "rt_looks_switch_costume_to",
            void_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &looks_switch_costume_to,
            rt_looks_switch_costume_to as *const () as usize,
        );

        let looks_switch_backdrop_to = module.add_function(
            "rt_looks_switch_backdrop_to",
            void_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &looks_switch_backdrop_to,
            rt_looks_switch_backdrop_to as *const () as usize,
        );

        let looks_set_effect_to = module.add_function(
            "rt_looks_set_effect_to",
            void_type.fn_type(&[ptr_type.into(), f64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &looks_set_effect_to,
            rt_looks_set_effect_to as *const () as usize,
        );

        let looks_set_size = module.add_function(
            "rt_looks_set_size",
            void_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine
            .add_global_mapping(&looks_set_size, rt_looks_set_size as *const () as usize);

        let looks_costume_number = module.add_function(
            "rt_looks_costume_number",
            f64_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &looks_costume_number,
            rt_looks_costume_number as *const () as usize,
        );

        let looks_costume_name = module.add_function(
            "rt_looks_costume_name",
            f64_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &looks_costume_name,
            rt_looks_costume_name as *const () as usize,
        );

        let looks_hide = module.add_function(
            "rt_looks_hide",
            void_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&looks_hide, rt_looks_hide as *const () as usize);

        let looks_show = module.add_function(
            "rt_looks_show",
            void_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&looks_show, rt_looks_show as *const () as usize);

        let music_set_tempo = module.add_function(
            "rt_music_set_tempo",
            void_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine
            .add_global_mapping(&music_set_tempo, rt_music_set_tempo as *const () as usize);

        let sensing_ask_and_wait = module.add_function(
            "rt_sensing_ask_and_wait",
            void_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &sensing_ask_and_wait,
            rt_sensing_ask_and_wait as *const () as usize,
        );

        let sensing_answer = module.add_function(
            "rt_sensing_answer",
            f64_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine
            .add_global_mapping(&sensing_answer, rt_sensing_answer as *const () as usize);

        let sensing_of = module.add_function(
            "rt_sensing_of",
            f64_type.fn_type(&[ptr_type.into(), f64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&sensing_of, rt_sensing_of as *const () as usize);

        let sensing_current = module.add_function(
            "rt_sensing_current",
            f64_type.fn_type(&[ptr_type.into(), i64_type.into()], false),
            None,
        );
        execution_engine
            .add_global_mapping(&sensing_current, rt_sensing_current as *const () as usize);

        let sensing_timer = module.add_function(
            "rt_sensing_timer",
            f64_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&sensing_timer, rt_sensing_timer as *const () as usize);

        let sensing_days_since_2000 = module.add_function(
            "rt_sensing_days_since_2000",
            f64_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &sensing_days_since_2000,
            rt_sensing_days_since_2000 as *const () as usize,
        );

        let sensing_touching_object = module.add_function(
            "rt_sensing_touching_object",
            f64_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &sensing_touching_object,
            rt_sensing_touching_object as *const () as usize,
        );

        let sensing_touching_color = module.add_function(
            "rt_sensing_touching_color",
            f64_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &sensing_touching_color,
            rt_sensing_touching_color as *const () as usize,
        );

        let sensing_reset_timer = module.add_function(
            "rt_sensing_reset_timer",
            void_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &sensing_reset_timer,
            rt_sensing_reset_timer as *const () as usize,
        );

        let pen_down = module.add_function(
            "rt_pen_down",
            void_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&pen_down, rt_pen_down as *const () as usize);

        let pen_up = module.add_function(
            "rt_pen_up",
            void_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&pen_up, rt_pen_up as *const () as usize);

        let pen_clear = module.add_function(
            "rt_pen_clear",
            void_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&pen_clear, rt_pen_clear as *const () as usize);

        let pen_set_size = module.add_function(
            "rt_pen_set_size",
            void_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&pen_set_size, rt_pen_set_size as *const () as usize);

        let pen_set_color = module.add_function(
            "rt_pen_set_color",
            void_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&pen_set_color, rt_pen_set_color as *const () as usize);

        let pen_stamp = module.add_function(
            "rt_pen_stamp",
            void_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&pen_stamp, rt_pen_stamp as *const () as usize);

        let pen_set_color_param = module.add_function(
            "rt_pen_set_color_param",
            void_type.fn_type(&[ptr_type.into(), i64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &pen_set_color_param,
            rt_pen_set_color_param as *const () as usize,
        );

        let control_create_clone = module.add_function(
            "rt_control_create_clone_of",
            void_type.fn_type(&[ptr_type.into(), i64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &control_create_clone,
            rt_control_create_clone_of as *const () as usize,
        );

        let control_delete_clone = module.add_function(
            "rt_control_delete_this_clone",
            void_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &control_delete_clone,
            rt_control_delete_this_clone as *const () as usize,
        );

        let control_stop = module.add_function(
            "rt_control_stop",
            void_type.fn_type(&[ptr_type.into(), i64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&control_stop, rt_control_stop as *const () as usize);

        let control_wait = module.add_function(
            "rt_control_wait",
            void_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&control_wait, rt_control_wait as *const () as usize);

        let repeat_count = module.add_function(
            "rt_repeat_count",
            i64_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&repeat_count, rt_repeat_count as *const () as usize);

        let operator_length = module.add_function(
            "rt_operator_length",
            f64_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine
            .add_global_mapping(&operator_length, rt_operator_length as *const () as usize);

        let operator_join = module.add_function(
            "rt_operator_join",
            f64_type.fn_type(&[ptr_type.into(), f64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&operator_join, rt_operator_join as *const () as usize);

        let operator_contains = module.add_function(
            "rt_operator_contains",
            f64_type.fn_type(&[ptr_type.into(), f64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &operator_contains,
            rt_operator_contains as *const () as usize,
        );

        let operator_round = module.add_function(
            "rt_operator_round",
            f64_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine
            .add_global_mapping(&operator_round, rt_operator_round as *const () as usize);

        let operator_letter_of = module.add_function(
            "rt_operator_letter_of",
            f64_type.fn_type(&[ptr_type.into(), f64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &operator_letter_of,
            rt_operator_letter_of as *const () as usize,
        );

        let operator_mathop = module.add_function(
            "rt_operator_mathop",
            f64_type.fn_type(&[ptr_type.into(), f64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine
            .add_global_mapping(&operator_mathop, rt_operator_mathop as *const () as usize);

        let operator_add = module.add_function(
            "rt_operator_add",
            f64_type.fn_type(&[ptr_type.into(), f64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&operator_add, rt_operator_add as *const () as usize);

        let operator_subtract = module.add_function(
            "rt_operator_subtract",
            f64_type.fn_type(&[ptr_type.into(), f64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &operator_subtract,
            rt_operator_subtract as *const () as usize,
        );

        let operator_multiply = module.add_function(
            "rt_operator_multiply",
            f64_type.fn_type(&[ptr_type.into(), f64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &operator_multiply,
            rt_operator_multiply as *const () as usize,
        );

        let operator_divide = module.add_function(
            "rt_operator_divide",
            f64_type.fn_type(&[ptr_type.into(), f64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine
            .add_global_mapping(&operator_divide, rt_operator_divide as *const () as usize);

        let operator_mod = module.add_function(
            "rt_operator_mod",
            f64_type.fn_type(&[ptr_type.into(), f64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&operator_mod, rt_operator_mod as *const () as usize);

        let operator_equals = module.add_function(
            "rt_operator_equals",
            f64_type.fn_type(&[ptr_type.into(), f64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine
            .add_global_mapping(&operator_equals, rt_operator_equals as *const () as usize);

        let operator_greater_than = module.add_function(
            "rt_operator_greater_than",
            f64_type.fn_type(&[ptr_type.into(), f64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &operator_greater_than,
            rt_operator_greater_than as *const () as usize,
        );

        let operator_less_than = module.add_function(
            "rt_operator_less_than",
            f64_type.fn_type(&[ptr_type.into(), f64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &operator_less_than,
            rt_operator_less_than as *const () as usize,
        );

        let data_item_of_list = module.add_function(
            "rt_data_item_of_list",
            f64_type.fn_type(&[ptr_type.into(), i64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &data_item_of_list,
            rt_data_item_of_list as *const () as usize,
        );

        let data_item_num_of_list = module.add_function(
            "rt_data_item_num_of_list",
            f64_type.fn_type(&[ptr_type.into(), i64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &data_item_num_of_list,
            rt_data_item_num_of_list as *const () as usize,
        );

        let data_length_of_list = module.add_function(
            "rt_data_length_of_list",
            f64_type.fn_type(&[ptr_type.into(), i64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &data_length_of_list,
            rt_data_length_of_list as *const () as usize,
        );

        let sensing_mouse_x = module.add_function(
            "rt_sensing_mouse_x",
            f64_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine
            .add_global_mapping(&sensing_mouse_x, rt_sensing_mouse_x as *const () as usize);

        let sensing_mouse_y = module.add_function(
            "rt_sensing_mouse_y",
            f64_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine
            .add_global_mapping(&sensing_mouse_y, rt_sensing_mouse_y as *const () as usize);

        let sensing_mouse_down = module.add_function(
            "rt_sensing_mouse_down",
            f64_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &sensing_mouse_down,
            rt_sensing_mouse_down as *const () as usize,
        );

        let sensing_key_pressed = module.add_function(
            "rt_sensing_key_pressed",
            f64_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &sensing_key_pressed,
            rt_sensing_key_pressed as *const () as usize,
        );

        let event_broadcast_value = module.add_function(
            "rt_event_broadcast_value",
            void_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &event_broadcast_value,
            rt_event_broadcast_value as *const () as usize,
        );

        let event_broadcast_wait_value = module.add_function(
            "rt_event_broadcast_and_wait_value",
            void_type.fn_type(&[ptr_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &event_broadcast_wait_value,
            rt_event_broadcast_and_wait_value as *const () as usize,
        );

        let forever_should_continue = module.add_function(
            "rt_forever_should_continue",
            bool_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &forever_should_continue,
            rt_forever_should_continue as *const () as usize,
        );

        let forever_should_continue_warp = module.add_function(
            "rt_forever_should_continue_warp",
            bool_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &forever_should_continue_warp,
            rt_forever_should_continue_warp as *const () as usize,
        );

        let loop_should_continue = module.add_function(
            "rt_loop_should_continue",
            bool_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &loop_should_continue,
            rt_loop_should_continue as *const () as usize,
        );

        let loop_should_continue_warp = module.add_function(
            "rt_loop_should_continue_warp",
            bool_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(
            &loop_should_continue_warp,
            rt_loop_should_continue_warp as *const () as usize,
        );

        let warp_enter = module.add_function(
            "rt_warp_enter",
            void_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&warp_enter, rt_warp_enter as *const () as usize);

        let warp_leave = module.add_function(
            "rt_warp_leave",
            void_type.fn_type(&[ptr_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&warp_leave, rt_warp_leave as *const () as usize);

        let random = module.add_function(
            "rt_random",
            f64_type.fn_type(&[ptr_type.into(), f64_type.into(), f64_type.into()], false),
            None,
        );
        execution_engine.add_global_mapping(&random, rt_random as *const () as usize);

        Self {
            count_executed_block,
            move_steps,
            set_direction,
            change_x,
            change_y,
            set_x,
            set_y,
            goto_xy,
            get_var,
            set_var,
            change_var,
            get_variables_ptr,
            data_add_to_list,
            data_delete_of_list,
            data_delete_all_of_list,
            data_replace_item_of_list,
            data_list_contains_item,
            say_number,
            say_text,
            looks_switch_costume_to,
            looks_switch_backdrop_to,
            looks_set_effect_to,
            looks_set_size,
            looks_costume_number,
            looks_costume_name,
            looks_hide,
            looks_show,
            music_set_tempo,
            sensing_ask_and_wait,
            sensing_answer,
            sensing_of,
            sensing_current,
            sensing_timer,
            sensing_days_since_2000,
            sensing_touching_object,
            sensing_touching_color,
            sensing_reset_timer,
            pen_down,
            pen_up,
            pen_clear,
            pen_set_size,
            pen_set_color,
            pen_stamp,
            pen_set_color_param,
            control_create_clone,
            control_delete_clone,
            control_stop,
            control_wait,
            repeat_count,
            operator_length,
            operator_join,
            operator_contains,
            operator_round,
            operator_letter_of,
            operator_mathop,
            operator_add,
            operator_subtract,
            operator_multiply,
            operator_divide,
            operator_mod,
            operator_equals,
            operator_greater_than,
            operator_less_than,
            data_item_of_list,
            data_item_num_of_list,
            data_length_of_list,
            motion_x_position,
            motion_y_position,
            sensing_mouse_x,
            sensing_mouse_y,
            sensing_mouse_down,
            sensing_key_pressed,
            event_broadcast_value,
            event_broadcast_wait_value,
            forever_should_continue,
            forever_should_continue_warp,
            loop_should_continue,
            loop_should_continue_warp,
            warp_enter,
            warp_leave,
            random,
        }
    }
}
