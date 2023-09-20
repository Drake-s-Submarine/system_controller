use super::{ COMMAND_QUEUE, Module };

pub fn dispatch_next_command(sub: &mut crate::Submarine) {
    let wrapper = COMMAND_QUEUE.lock().unwrap().pop_front();

    match wrapper {
        Some(w) => {
            match w.module {

                Module::Ballast => {
                    let cmd =
                        std::mem::ManuallyDrop::into_inner(
                            unsafe{w.command.ballast}
                        );

                    sub.ballast.handle_command(cmd.as_ref());
                },
                Module::Light => {
                    let cmd =
                        std::mem::ManuallyDrop::into_inner(
                            unsafe{w.command.light}
                        );

                    sub.light.handle_command(cmd.as_ref());
                },
                Module::Propulsion => {
                    let cmd =
                        std::mem::ManuallyDrop::into_inner(
                            unsafe{w.command.propulsion}
                        );

                    sub.propulsion.handle_command(cmd.as_ref());
                },

            }
        },
        None => {}
    }
}
