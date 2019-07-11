// https://github.com/DenisKolodin/yew/tree/master/examples
// https://github.com/anxiousmodernman/rocket-yew-starter-pack/blob/master/ui/src/lib.rs
// https://github.com/saschagrunert/webapp.rs/tree/rev1/src/frontend/components
// https://github.com/deciduously/hunt-the-wumpus/tree/master/part3/src

extern crate yew;

extern crate sl_lib;

use sl_lib::Model;
use yew::prelude::App;

fn main() {
  yew::initialize();
  let app: App<Model> = App::new();
  app.mount_to_body();
  yew::run_loop();
}
