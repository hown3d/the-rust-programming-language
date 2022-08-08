mod channels;
mod thread;
fn main() {
    println!("Executing func: wait_for_thread_to_finish");
    thread::wait_for_thread_to_finish();
    println!("Executing func: moving_data_into_thread_closure");
    thread::moving_data_into_thread_closure();
    println!("Executing func: channels");
    channels::channels();
    println!("Executing func: channel_as_iterator");
    channels::channel_as_iterator();
    println!("Executing func: channel_multiple_senders");
    channels::channel_multiple_senders();
}
