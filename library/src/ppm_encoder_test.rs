use demonstrate::demonstrate;
use indoc::indoc;

use crate::color::Color;
use crate::PpmEncoder;
use crate::Sdl2Interface;

demonstrate! {
    describe "PpmEncoder" {
        use super::*;

        it "should encode an exportable image" {
            let mut sdl_interface = Sdl2Interface::init("test", 5, 3, (0, 0));

            let c1 = Color::new(1.5, 0, 0);
            let c2 = Color::new(0, 0.5, 0);
            let c3 = Color::new(-0.5, 0, 1);

            sdl_interface.write_pixel(0, 0, c1);
            sdl_interface.write_pixel(2, 1, c2);
            sdl_interface.write_pixel(4, 2, c3);

            let mut buffer_bytes = Vec::new();

            PpmEncoder::export_image(&sdl_interface, &mut buffer_bytes);

            let buffer_string = String::from_utf8(buffer_bytes).unwrap();

            let expected_string = indoc! {"
                P3
                5 3
                255
                0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 255 0
                0 0 0 0 0 0 0 0 0 0 0 0 0
            "};

            assert_eq!(buffer_string, expected_string);
        }
    }
}
