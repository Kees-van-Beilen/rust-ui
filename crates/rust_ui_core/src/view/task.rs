//tasks

use std::thread::{self, JoinHandle};


pub struct Task {
    /// This function will always run on the main thread
    handle:JoinHandle<()>
}


impl Task {

    /// 
    /// When not using macro's, use this method to create and directly run.
    /// The executor will be ran on a different thread from the main/ui thread
    /// , and may even run indefinitely. If the executor finishes, a flush is called.
    /// This flush always runs on the main thead, using platform specific unsafe sorcery.
    /// In this flush the sync function gets called. This function, because it runs on the main thread,
    /// may update UI components, and doesn't require to be sync
    /// 
    /// One problem with tasks currently:
    /// They can't externally be suspended/killed
    /// the flush function can introspect if the current is actually rendered
    /// and the notify the executor via a msg or arc mutex. It is however still
    /// the responsibility of the executor the stop execution appropriately.
    /// 
    /// This is only a problem for long running/indefinite executions.
    /// 
    /// There are also some annoying android restrictions I haven't looked at 
    /// (having to do with the jnienv). 
    /// 
    /// 
    pub fn create<A:Send+'static,Exe:FnOnce(&dyn Fn(A))->A+Send+'static,Syn:Fn(A)+Clone+'static>(executor:Exe,sync:Syn)->Self{
        let flush = crate::native::create_task_flush(sync);

        let handle = thread::spawn(move ||{
            let flush_dyn = &flush;
            let a = executor(flush_dyn);
            flush(a);
        });
        Task{
            handle
        }
    }
}