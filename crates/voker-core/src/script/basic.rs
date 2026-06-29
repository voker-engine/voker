// use alloc::boxed::Box;

// pub enum ScriptMut<'a> {
//     World(&'a mut dyn WorldScript),
//     Entity(&'a mut dyn EntityScript),
// }

// pub enum Script {
//     Static(&'static mut dyn BasicScript),
//     Boxed(Box<dyn BasicScript>)
// }

// pub trait BasicScript {
//     fn path(&self) -> &'static str;
//     fn clone(&self) -> Script;
// }

// pub trait WorldScript {
//     // fn run(&mut self, &mut World);
//     // fn notate(&self, out: &mut AccessInfoBuilder);
// }

// pub trait EntityScript {
//     // fn run(&mut self, EntityMut);
//     // fn is_readonly(&self) -> bool;
//     // fn is_isolated(&self) -> bool;
// }
