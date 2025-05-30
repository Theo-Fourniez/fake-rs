use std::ops::Range;

use geo_types::CoordNum;
use rand::Rng;

use crate::{Dummy, Fake, Faker};

impl<T: CoordNum + Dummy<Faker>> Dummy<Faker> for geo_types::Coord<T> {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        geo_types::Coord::<T> {
            x: Faker.fake_with_rng(rng),
            y: Faker.fake_with_rng(rng),
        }
    }
}

impl<T: CoordNum + Dummy<Faker>> Dummy<Faker> for geo_types::Line<T> {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        geo_types::Line::<T> {
            start: Faker.fake_with_rng(rng),
            end: Faker.fake_with_rng(rng),
        }
    }
}

impl<T: CoordNum + Dummy<Faker>> Dummy<Faker> for geo_types::LineString<T> {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        geo_types::LineString::<T>::new(Faker.fake_with_rng(rng))
    }
}

impl<T: CoordNum + Dummy<Faker>> Dummy<Faker> for geo_types::MultiLineString<T> {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        geo_types::MultiLineString::<T>::new(Faker.fake_with_rng(rng))
    }
}

impl<T: CoordNum + Dummy<Faker>> Dummy<Faker> for geo_types::Point<T> {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        geo_types::Point::<T>::new(Faker.fake_with_rng(rng), Faker.fake_with_rng(rng))
    }
}

impl<T: CoordNum + Dummy<Faker>> Dummy<Faker> for geo_types::MultiPoint<T> {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        geo_types::MultiPoint::<T>::new(Faker.fake_with_rng(rng))
    }
}

impl<T: CoordNum + Dummy<Faker>> Dummy<Faker> for geo_types::Polygon<T> {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        // Polygon will auto-close these LineString.
        geo_types::Polygon::<T>::new(Faker.fake_with_rng(rng), Faker.fake_with_rng(rng))
    }
}

impl<T: CoordNum + Dummy<Faker>> Dummy<Faker> for geo_types::MultiPolygon<T> {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        geo_types::MultiPolygon::<T>::new(Faker.fake_with_rng(rng))
    }
}

impl<T: CoordNum + Dummy<Faker>> Dummy<Faker> for geo_types::Rect<T> {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        // Rect points can not overlap.
        let nums: Vec<T> = crate::unique::<T, _>(rng, 4);
        let coord_1 = geo_types::Coord::<T> {
            x: nums[0],
            y: nums[1],
        };
        let coord_2 = geo_types::Coord::<T> {
            x: nums[2],
            y: nums[3],
        };
        geo_types::Rect::new::<geo_types::Coord<T>>(coord_1, coord_2)
    }
}

impl<T: CoordNum + Dummy<Faker>> Dummy<Faker> for geo_types::Triangle<T> {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        fn step<T, R: Rng + ?Sized>(start: T, steps: usize, rng: &mut R) -> T
        where
            T: CoordNum,
        {
            let mut current = start;
            for i in 0..steps {
                current = current + T::one();
                if i > 1 && Faker.fake_with_rng(rng) {
                    break;
                }
            }
            current
        }
        let coord_1 = geo_types::Coord::<T> {
            x: step(T::zero(), 10, rng),
            y: step(T::zero(), 10, rng),
        };
        let mut coord_2 = geo_types::Coord::<T> {
            x: step(coord_1.x, 10, rng),
            y: step(coord_1.y, 10, rng),
        };
        let mut coord_3 = geo_types::Coord::<T> {
            x: step(coord_1.x, 10, rng),
            y: step(coord_2.y, 5, rng),
        };
        if coord_2.x < coord_3.x {
            let tmp = coord_2.x;
            coord_2.x = coord_3.x;
            coord_3.x = tmp;
        }
        geo_types::Triangle::<T>::new(coord_1, coord_2, coord_3)
    }
}

const GEOMETRY_UNION_MEMBERS_IGNORE_RECURSIVE: usize = 7;
const GEOMETRY_UNION_MEMBERS: Range<usize> = 0..9;

// The GeometryCollection cant include a Geometry which is a GeometryCollection
// to avoid overflowing the stack.
struct NonRecursiveGeometry<T: CoordNum = f64>(geo_types::Geometry<T>);

impl<T: CoordNum + Dummy<Faker>> Dummy<Faker> for NonRecursiveGeometry<T> {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let union_index: usize = GEOMETRY_UNION_MEMBERS.fake_with_rng(rng);
        match union_index {
            0 => NonRecursiveGeometry(geo_types::geometry::Geometry::Point::<T>(
                Faker.fake_with_rng(rng),
            )),
            1 => NonRecursiveGeometry(geo_types::geometry::Geometry::Line::<T>(
                Faker.fake_with_rng(rng),
            )),
            2 => NonRecursiveGeometry(geo_types::geometry::Geometry::LineString::<T>(
                Faker.fake_with_rng(rng),
            )),
            3 => NonRecursiveGeometry(geo_types::geometry::Geometry::Polygon::<T>(
                Faker.fake_with_rng(rng),
            )),
            4 => NonRecursiveGeometry(geo_types::geometry::Geometry::MultiPoint::<T>(
                Faker.fake_with_rng(rng),
            )),
            5 => NonRecursiveGeometry(geo_types::geometry::Geometry::MultiLineString::<T>(
                Faker.fake_with_rng(rng),
            )),
            6 => NonRecursiveGeometry(geo_types::geometry::Geometry::MultiPolygon::<T>(
                Faker.fake_with_rng(rng),
            )),
            // Replace GeometryCollection with a Point
            GEOMETRY_UNION_MEMBERS_IGNORE_RECURSIVE => NonRecursiveGeometry(
                geo_types::geometry::Geometry::Point::<T>(Faker.fake_with_rng(rng)),
            ),
            8 => NonRecursiveGeometry(geo_types::geometry::Geometry::Rect::<T>(
                Faker.fake_with_rng(rng),
            )),
            9 => NonRecursiveGeometry(geo_types::geometry::Geometry::Triangle::<T>(
                Faker.fake_with_rng(rng),
            )),
            _ => panic!(),
        }
    }
}

impl<T: CoordNum + Dummy<Faker>> Dummy<Faker> for geo_types::Geometry<T> {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let union_index: usize = GEOMETRY_UNION_MEMBERS.fake_with_rng(rng);
        match union_index {
            0 => geo_types::geometry::Geometry::Point::<T>(Faker.fake_with_rng(rng)),
            1 => geo_types::geometry::Geometry::Line::<T>(Faker.fake_with_rng(rng)),
            2 => geo_types::geometry::Geometry::LineString::<T>(Faker.fake_with_rng(rng)),
            3 => geo_types::geometry::Geometry::Polygon::<T>(Faker.fake_with_rng(rng)),
            4 => geo_types::geometry::Geometry::MultiPoint::<T>(Faker.fake_with_rng(rng)),
            5 => geo_types::geometry::Geometry::MultiLineString::<T>(Faker.fake_with_rng(rng)),
            6 => geo_types::geometry::Geometry::MultiPolygon::<T>(Faker.fake_with_rng(rng)),
            7 => geo_types::geometry::Geometry::GeometryCollection::<T>(Faker.fake_with_rng(rng)),
            8 => geo_types::geometry::Geometry::Rect::<T>(Faker.fake_with_rng(rng)),
            9 => geo_types::geometry::Geometry::Triangle::<T>(Faker.fake_with_rng(rng)),
            _ => panic!(),
        }
    }
}

impl<T: CoordNum + Dummy<Faker>> Dummy<Faker> for geo_types::GeometryCollection<T> {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        geo_types::GeometryCollection::<T>::new_from(
            Faker
                .fake_with_rng::<Vec<NonRecursiveGeometry<T>>, _>(rng)
                .iter()
                .map(|x| (x.0.to_owned()))
                .collect::<Vec<geo_types::Geometry<T>>>(),
        )
    }
}
