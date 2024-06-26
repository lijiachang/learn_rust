mod loop_while_for;
mod string;
mod ownership_and_method;
mod ownership_and_return;
mod ownership_and_method2;
mod ref_calculate_len;
mod ref_calculate_len2;
mod slice;
mod struct_;
mod tuple_struct;
mod struct_area;
mod struct_method;
mod enum_;
mod enum_option;
mod match_;
mod match_2;
mod match_3;
mod if_let;
mod vector_;
mod string_2;
mod hashmap;
mod panic_;
mod trait_;
mod trait_as_arg;
mod lifetime;
mod closure;
mod iterator;
mod smart_points;
mod smart_points_deref;
mod smart_points_drop;
mod smart_points_rc;
mod smart_points_refcell;
mod thread;
mod thread_mpsc;
mod thread_mutex;
mod thread_send_and_sync;
mod oop;
mod oop_state_pattern;
mod pattern_math;
mod unsafe_;
mod advanced_trait;
mod adcanced_type;
mod advanced_fn_and_closure;
mod async_1;
mod async_tokio_select;
mod async_tokio_broadcast;
mod async_tokio_joinSet;
mod async_tokio_RwLock;
mod async_stream_yield;
mod file_loop;
mod decimal;
mod for_a_syntax;
mod lifetime_2;
mod closure_2;
mod smart_points_2;
mod trait_polymorphism;
mod trait_operators;
mod arc_mutex;


fn main() {
    loop_while_for::loop_();
    loop_while_for::while_();
    loop_while_for::for_();
    loop_while_for::range_();
    string::main();
    string::show_move();
    ownership_and_method2::main();
    ref_calculate_len::main();
    slice::main();
    struct_::main();
    struct_area::main();
    struct_method::main();
    enum_::ip();
    match_::main();
    match_2::main();
    vector_::update_element_when_for_loop_vector();
    string_2::main();
    hashmap::main();
    panic_::main();
    trait_::main();
    lifetime::main();
    lifetime::test_longest_with_announcement();
    // closure::main();
    smart_points::main();
    smart_points_deref::main();
    smart_points_drop::main();
    smart_points_rc::main();
    thread::main();
    thread::move_();
    thread_mpsc::main();
    thread_mutex::main();
    oop::main();
    pattern_math::main();
    unsafe_::main();
    advanced_trait::vec_display();
    advanced_fn_and_closure::main();
    // async_1::main();
    // file_loop::main();
    decimal::main();
}