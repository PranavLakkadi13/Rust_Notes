mod message_passing;
mod mutexes_concurrency;
mod scoping_to_threads;
mod test_intro_threads;

fn main() {
    // test_intro_threads::public_call_explore();
    // test_intro_threads::test_my_thread();
    // test_intro_threads::spawing_my_thread();

    // scoping_to_threads::test_scoping_threads();

    // mutexes_concurrency::mutex_share();
    // mutexes_concurrency::multi_thread();
    // mutexes_concurrency::multi_thread_sleep_when_notreceived_mutex();

    // message_passing::test();
    message_passing::test_seperate_thread_channel();
}
