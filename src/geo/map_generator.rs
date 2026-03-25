use std::collections::HashMap;
use osmpbf::{Element, ElementReader, IndexedReader};
use skia_safe::{Canvas, RGB};
use crate::motion_graphics::elements::Element as MotionElement;
use vector2d::Vector2D;
use crate::motion_graphics::attributes::attribute;
use crate::motion_graphics::attributes::type_extensions::InterpolationArithmetics;
use crate::motion_graphics::elements::line::Line;

pub struct MapReader {
    pub path: String,
    pub level_of_detail: u8,
}

pub struct Map{
    pub position: Box<dyn attribute::Attribute<Vector2D<f32>>>,
    pub scale: Box<dyn attribute::Attribute<f32>>,
    pub data: MapData,
}

pub struct MapData{
    pub map: HashMap<i64,WayData>,
    pub center_point: Vector2D<f64>,
    pub max_point: Vector2D<f64>,
    pub min_point: Vector2D<f64>,
}

pub struct WayData{
    pub id: i64,
    pub way_points: Vec<Vector2D<f64>>,
    pub max_points: Vector2D<f64>,
    pub min_points: Vector2D<f64>,
}

impl MapReader {
    pub fn import_osm_file(&self) -> MapData{

        let ways = MapReader::open_osm(&self.path, self.level_of_detail);

        let (center_point , max_points, min_points)= {
            let mut x_max = -f64::INFINITY;
            let mut x_min = f64::INFINITY;
            let mut y_max = -f64::INFINITY;
            let mut y_min = f64::INFINITY;
            for (_, value) in &ways{
                for value in value.way_points.iter(){
                    if x_max < value.x {
                        x_max = value.x;
                    }
                    if y_max < value.y {
                        y_max = value.y;
                    }
                    if x_min > value.x {
                        x_min = value.x;
                    }
                    if y_min > value.y {
                        y_min = value.y;
                    }
                }
            }
            let x_center = (x_min + x_max) / 2f64;
            let y_center = (y_min + y_max) / 2f64;

            (Vector2D::<f64>::new(x_center,y_center), Vector2D::<f64>::new(x_max,y_max),Vector2D::new(x_min,y_min))
        };

        MapData{
            map: ways,
            center_point,
            max_point: max_points,
            min_point: min_points,
        }
    }

    fn open_osm(path: &String, level_of_detail: u8) -> HashMap<i64, WayData>{
        let mut reader = IndexedReader::from_path(path).unwrap();
        let mut way_refs : HashMap<i64,Vec<i64>> = HashMap::new();
        let mut nodes : HashMap<i64, Vector2D<f64>> = HashMap::new();

        reader.read_ways_and_deps(|way| true, |element|{
            match element{
                Element::Relation(rel) => {
                    rel.members().for_each(|member|{
                        println!("{}",member.member_id);
                    })
                }

                Element::Way(way) => {
                    if way.refs().len() > (255 / level_of_detail) as usize{
                        let mut _way = Vec::<i64>::new();
                        for nid in way.refs() {
                            _way.push(nid);
                        }
                        way_refs.insert(way.id(),_way);
                    };
                }
                Element::Node(_n) => {
                    nodes.insert(_n.id(),Vector2D::<f64>::new(_n.lat(),_n.lon()));
                }
                Element::DenseNode(_n) => {
                    nodes.insert(_n.id(),Vector2D::<f64>::new(_n.lat(),_n.lon()));
                }
                _ => {}
            }
        }).unwrap();

        let mut ways : HashMap<i64,WayData> = HashMap::new();

        for (key, ref_nodes) in way_refs.iter() {
            let mut way = Vec::new();
            let mut x_max = -f64::INFINITY;
            let mut x_min = f64::INFINITY;
            let mut y_max = -f64::INFINITY;
            let mut y_min = f64::INFINITY;

            for ref_node in ref_nodes {

                let value =nodes[ref_node];
                if x_max < value.x {
                    x_max = value.x;
                }
                if y_max < value.y {
                    y_max = value.y;
                }
                if x_min > value.x {
                    x_min = value.x;
                }
                if y_min > value.y {
                    y_min = value.y;
                }

                way.push(nodes[ref_node]);
            }


            let data = WayData{
                id: *key,
                way_points: way,
                max_points: Vector2D::new(x_max,y_max),
                min_points: Vector2D::new(x_min,y_min),
            };
            ways.insert(*key,data);
        }
        ways
    }
}

impl MotionElement for Map{
    fn draw_on(&self, frame: usize, canvas: &Canvas) -> Result<(), &'static str> {

        let scale = self.scale.get_frame(frame);

        for (k,v) in self.data.map.iter() {
            let mut points = Vec::<Box<dyn attribute::Attribute<Vector2D<f32>>>>::new();
            for wp in v.way_points.iter() {
                let x = (wp.x - self.data.center_point.x) * scale as f64;
                let y = (wp.y - self.data.center_point.y) * scale as f64;

                points.push(Vector2D::new(x as f32, y as f32).into_bsa())
            }

            Line {
                position_offset: self.position.clone(),
                start: 0f32.into_bsa(),
                end: 0f32.into_bsa(),
                width: 1.0f32.into_bsa(),
                color: RGB{r:200, g: 200, b: 200}.into_bsa(),
                stroke_caps: skia_safe::paint::Cap::Round,
                is_antialias: true,
                points,
            }.draw_on(frame, canvas)?;
        };

        Ok(())
    }
}