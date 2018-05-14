#[derive(Clone)]
struct Object {
    ref_count: i64 // std::atomic<int> m_refCount
}

impl Object {
    fn new() -> Object { Object { ref_count: 0 } }
    fn get_ref_count(&self) -> i64 { self.ref_count }
    fn inc_ref(&mut self) { self.ref_count += 1; }
    fn dec_ref(&mut self, dealloc: bool) {
        self.ref_count -= 1;
        if self.ref_count == 0 && dealloc {
            // delete this;
            ;
        } else if self.ref_count < 0 {
            panic!("Internal error: Object reference count < 0!\n");
            // fprintf(stderr, "Internal error: Object reference count < 0!\n");
            // abort();
        }
    }
}

struct NVGcontext {}

// Eigen::Vector2i らしいけどどうしよう
// ひとまず自作してみた
struct Vector2i {
    x: i64,
    y: i64,
}

trait Layout {
    fn perform_layout(ctx: Box<NVGcontext>, widget: Box<Widget>);
    fn preferred_size(ctx: Box<NVGcontext>, widget: Box<Widget>) -> Vector2i;
}

#[derive(Clone)]
struct Widget {
    parent: Option<Box<Widget>>
}

impl Widget {
    fn new(parent: Box<Widget>) -> Option<Box<Widget>> { Some(parent) }

    fn get_parent(&self) -> Option<Box<Widget>> { self.parent.clone() }
    fn set_parent(&mut self, parent: Box<Widget>) { self.parent = Some(parent) }


}

fn main() {
    println!("Hello, world!");
}
