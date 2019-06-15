//! Debug helpers for printing various things to the debug output

use gtk::Builder;
use gtk::BuilderExt;
use gtk::Cast;

pub fn print_all_objects(builder: &gtk::Builder) {
    for obj in builder.get_objects() {
        debug!("Builder has object: {:?}", obj);
        obj.downcast::<gtk::MenuItem>()
            .map(|mitem| {
                debug!("-> MenuItem: {:?}", mitem);
            });
    }
}
