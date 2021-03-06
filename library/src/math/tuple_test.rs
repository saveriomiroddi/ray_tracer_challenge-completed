use demonstrate::demonstrate;

demonstrate! {
    describe "Tuple" {
        use crate::*;
        use crate::math::{*, tuple::{POINT_TYPE, VECTOR_TYPE}};
        use crate::lang::{math::sqrt, ApproximateFloat64Ops};
        use std::f64::consts::PI;

        context "with w=1_0" {
            it "is a point" {
                let tuple = Tuple { x: 2.0, y: 4.0, z: 8.0, w: 1.0 };

                match tuple {
                    Tuple { x: _, y: _, z: _, w: type_value } => {
                        assert_float_absolute_eq!(type_value, POINT_TYPE);
                    }
                }
            }
        } // context "with w=1_0"

        context "with w=0_0" {
            it "is a vector" {
                let tuple = Tuple { x: 2.0, y: 4.0, z: 8.0, w: 0.0 };

                match tuple {
                    Tuple { x: _, y: _, z: _, w: type_value } => {
                        assert_float_absolute_eq!(type_value, VECTOR_TYPE);
                    }
                }
            }
        } // context "with w=0_0"

        context "::point" {
            it "creates a tuple with w=1_0" {
                let tuple = Tuple::point(2.0, 4.0, 8.0);

                match tuple {
                    Tuple { x: _, y: _, z: _, w: type_value } => {
                        assert_float_absolute_eq!(type_value, POINT_TYPE);
                    }
                }
            }
        } // context "::point"

        context "::vector" {
            it "creates a tuple with w=0_0" {
                let tuple = Tuple::vector(2.0, 4.0, 8.0);

                match tuple {
                    Tuple { x: _, y: _, z: _, w: type_value } => {
                        assert_float_absolute_eq!(type_value, VECTOR_TYPE);
                    }
                }
            }
        } // context "::vector"

        // For simplicity, ignore NaN.
        //
        it "equals other tuples with the same values, within epsilon" {
            let tuple1 = Tuple { x: 1.0, y: 2.0, z: 3.0, w: 1.0 };
            let tuple2 = Tuple { x: 1.00000000001, y: 2.00000000001, z: 3.00000000001, w: 1.00000000001 };

            assert_eq!(tuple1, tuple2);
        }

        it "can be added to another tuple" {
            let tuple1 = Tuple { x: 3.0, y: -2.0, z: 5.0, w: 1.0 };
            let tuple2 = Tuple { x: -2.0, y: 3.0, z: 1.0, w: 0.0 };

            let expected_tuple = Tuple { x: 1.0, y: 1.0, z: 6.0, w: 1.0 };

            assert_eq!(tuple1 + &tuple2, expected_tuple);
        }

        context "as point" {
            it "can be subtracted from a point" {
                let tuple1 = Tuple::point(3.0, 2.0, 1.0);
                let tuple2 = Tuple::point(5.0, 6.0, 7.0);

                let expected_tuple = Tuple::vector(-2.0, -4.0, -6.0);

                assert_eq!(tuple1 - &tuple2, expected_tuple);
            }
        } // context "as point"

        context "as vector" {
            it "can be subtracted from a point" {
                let tuple1 = Tuple::point(3.0, 2.0, 1.0);
                let tuple2 = Tuple::vector(5.0, 6.0, 7.0);

                let expected_tuple = Tuple::point(-2.0, -4.0, -6.0);

                assert_eq!(tuple1 - &tuple2, expected_tuple);
            }

            it "can be subtracted from a vector" {
                let tuple1 = Tuple::vector(3.0, 2.0, 1.0);
                let tuple2 = Tuple::vector(5.0, 6.0, 7.0);

                let expected_tuple = Tuple::vector(-2.0, -4.0, -6.0);

                assert_eq!(tuple1 - &tuple2, expected_tuple);
            }

            it "can be normalized" {
                let vector = Tuple::vector(1.0, 2.0, 3.0);

                let expected_vector = Tuple::vector(1.0 / sqrt(14), 2.0 / sqrt(14), 3.0 / sqrt(14));

                assert_eq!(vector.normalize(), expected_vector);
            }

            it "has a cross product with another vector" {
                let vector1 = Tuple::vector(1.0, 2.0, 3.0);
                let vector2 = Tuple::vector(2.0, 3.0, 4.0);

                let expected_vector = Tuple::vector(-1.0, 2.0, -1.0);

                assert_eq!(vector1.cross_product(vector2), expected_vector);
            }
        } // context "as vector"

        it "can be subtracted from the zero vector" {
            let tuple1 = Tuple::vector(0.0, 0.0, 0.0);
            let tuple2 = Tuple::vector(5.0, 6.0, 7.0);

            let expected_tuple = Tuple::vector(-5.0, -6.0, -7.0);

            assert_eq!(tuple1 - &tuple2, expected_tuple);
        }

        // At this stage of the book, it's unclear why the book negates a non-meaningful tuple.
        //
        it "can be negated" {
            let tuple = Tuple { x: 1.0, y: -2.0, z: 0.0, w: 2.0 };

            let expected_tuple = Tuple { x: -1.0, y: 2.0, z: -0.0, w: -2.0 };

            assert_eq!(-tuple, expected_tuple);
        }

        it "can be multiplied by a floating point factor" {
            let tuple = Tuple { x: 1.0, y: -2.0, z: 0.0, w: 2.0 };

            let expected_tuple = Tuple { x: 2.5, y: -5.0, z: 0.0, w: 5.0 };

            assert_eq!(tuple * 2.5, expected_tuple);
        }

        it "can be divided by a floating point factor" {
            let tuple = Tuple { x: 1.0, y: -2.0, z: 0.0, w: 2.0 };

            let expected_tuple = Tuple { x: 2.0, y: -4.0, z: 0.0, w: 4.0 };

            assert_eq!(tuple / 0.5, expected_tuple);
        }

        context "should have a magnitude" {
            it "as vector (-1, -2, -3)" {
                let vector = Tuple::vector(-1.0, -2.0, -3.0);

                let expected_magnitude = sqrt(14);

                assert!(vector.magnitude().approximate_equals(expected_magnitude));
            }
        } // context "should have a magnitude"

        it "has a dot product" {
            let tuple1 = Tuple::vector(1.0, 2.0, 3.0);
            let tuple2 = Tuple::vector(2.0, 3.0, 4.0);

            let expected_dot_product = 20.0;

            assert_eq!(tuple1.dot_product(&tuple2), expected_dot_product);
        }

        context "transformations" {
            it "should translate" {
                let tuple = Tuple::point(-3, 4, 5);

                let expected_result = Tuple::point(2, 1, 7);

                assert_eq!(tuple.translate(5, -3, 2), expected_result);
            }

            it "should scale" {
                let tuple = Tuple::point(-4, 6, 8);

                let expected_result = Tuple::point(-8, 18, 32);

                assert_eq!(tuple.scale(2, 3, 4), expected_result);
            }

            context "rotation" {
                it "should by performed by Pi/4 around the x axis" {
                    let tuple = Tuple::point(0, 1, 0);

                    let expected_result = Tuple::point(0, sqrt(2) / 2.0, sqrt(2) / 2.0);

                    assert_eq!(tuple.rotate(Axis::X, PI / 4.0), expected_result);
                }

                it "should by performed by Pi/4 around the y axis" {
                    let tuple = Tuple::point(0, 0, 1);

                    let expected_result = Tuple::point(sqrt(2) / 2.0, 0, sqrt(2) / 2.0);

                    assert_eq!(tuple.rotate(Axis::Y, PI / 4.0), expected_result);
                }

                it "should by performed by Pi/4 around the z axis" {
                    let tuple = Tuple::point(0, 1, 0);

                    let expected_result = Tuple::point(-sqrt(2) / 2.0, sqrt(2) / 2.0, 0);

                    assert_eq!(tuple.rotate(Axis::Z, PI / 4.0), expected_result);
                }
            } // context "rotation"

            context "shearing" {
                it "should move x in proportion to y" {
                    let tuple = Tuple::point(2, 3, 4);

                    let expected_result = Tuple::point(5, 3, 4);

                    assert_eq!(tuple.shear(1, 0, 0, 0, 0, 0), expected_result);
                }

                it "should move x in proportion to z" {
                    let tuple = Tuple::point(2, 3, 4);

                    let expected_result = Tuple::point(6, 3, 4);

                    assert_eq!(tuple.shear(0, 1, 0, 0, 0, 0), expected_result);
                }

                it "should move y in proportion to x" {
                    let tuple = Tuple::point(2, 3, 4);

                    let expected_result = Tuple::point(2, 5, 4);

                    assert_eq!(tuple.shear(0, 0, 1, 0, 0, 0), expected_result);
                }

                it "should move y in proportion to z" {
                    let tuple = Tuple::point(2, 3, 4);

                    let expected_result = Tuple::point(2, 7, 4);

                    assert_eq!(tuple.shear(0, 0, 0, 1, 0, 0), expected_result);
                }

                it "should move z in proportion to x" {
                    let tuple = Tuple::point(2, 3, 4);

                    let expected_result = Tuple::point(2, 3, 6);

                    assert_eq!(tuple.shear(0, 0, 0, 0, 1, 0), expected_result);
                }

                it "should move z in proportion to y" {
                    let tuple = Tuple::point(2, 3, 4);

                    let expected_result = Tuple::point(2, 3, 7);

                    assert_eq!(tuple.shear(0, 0, 0, 0, 0, 1), expected_result);
                }
            } // context "shearing"

            it "should be applicable in sequence" {
                let tuple = Tuple::point(1, 0, 1);

                // "Not amazing" test data. After the rotation, the z value is 0, so that any scale
                // z value passes the UT.
                //
                let current_result = tuple
                    .rotate(Axis::X, PI / 2.0)
                    .scale(5, 5, 5)
                    .translate(10, 5, 7);

                let expected_result = Tuple::point(15, 0, 7);

                assert_eq!(current_result, expected_result);
            }
        } // context "transformations"

        it "should reflect" {
            let vector = Tuple::vector(1, -1, 0);
            let normal = Tuple::vector(0, 1, 0);

            let expected_reflection = Tuple::vector(1, 1, 0);

            assert_eq!(vector.reflect(&normal), expected_reflection);
        }
    }
}
