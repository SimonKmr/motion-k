use std::collections::HashMap;
use std::str::FromStr;
use osmpbf::{Element, ElementReader, IndexedReader};
use pixels::wgpu::naga::FastHashMap;
use skia_safe::{Canvas, RGB};
use crate::motion_graphics::elements::Element as MotionElement;
use vector2d::Vector2D;
use crate::geo::style::MapStyleSettings;
use crate::motion_graphics::attributes::attribute;
use crate::motion_graphics::attributes::type_extensions::InterpolationArithmetics;
use crate::motion_graphics::elements::line::Line;

pub struct MapReader {
    pub path: String,
    pub center_point: Option<Vector2D<f64>>,
    pub settings: MapSelectionSettings,
}

pub struct Map{
    pub position: Box<dyn attribute::Attribute<Vector2D<f32>>>,
    pub scale: Box<dyn attribute::Attribute<f32>>,
    pub data: MapData,
    pub settings: Option<MapStyleSettings>,
}

pub struct MapData{
    pub map: HashMap<i64,WayData>,
    pub center_point: Vector2D<f64>,
    pub max_point: Vector2D<f64>,
    pub min_point: Vector2D<f64>,
}

pub struct WayData{
    pub id: i64,
    pub highway_tag: String,
    pub way_points: Vec<Vector2D<f64>>,
    pub max_points: Vector2D<f64>,
    pub min_points: Vector2D<f64>,
}


pub struct MapSelectionSettings{
    pub way_settings: HashMap<String,bool>
}

impl MapReader {

    pub fn new(path: String, center_point: Option<Vector2D<f64>>, settings: Option<MapSelectionSettings>) -> MapReader {
        MapReader{
            path,
            center_point,
            settings: settings.unwrap_or_default(),
        }
    }

    pub fn import_osm_file(&self) -> MapData{

        let ways = self.open_osm();

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
            let center = self.center_point.unwrap_or(Vector2D::new((x_min + x_max) / 2f64,(y_min + y_max) / 2f64));

            (center, Vector2D::<f64>::new(x_max,y_max),Vector2D::new(x_min,y_min))
        };

        MapData{
            map: ways,
            center_point,
            max_point: max_points,
            min_point: min_points,
        }
    }

    fn open_osm(&self) -> HashMap<i64, WayData>{
        let mut reader = IndexedReader::from_path(&self.path).unwrap();
        let mut way_refs : HashMap<i64,(Vec<i64>,String)> = HashMap::new();
        let mut nodes : HashMap<i64, Vector2D<f64>> = HashMap::new();

        reader.read_ways_and_deps(|way| {
            for (key,value) in way.tags(){
                if key == "highway" {
                    let way_setting = &self.settings.way_settings;

                    return if way_setting.contains_key(&value.to_string()) {
                        way_setting[value]
                    } else {
                        false
                    }
                }
            }
            false
        }, |element|{
            match element{
                Element::Relation(rel) => {
                    rel.members().for_each(|member|{
                        println!("{}",member.member_id);
                    })
                }

                Element::Way(way) => {
                    let mut _way = Vec::<i64>::new();
                    let mut _way_type = String::new();
                    for nid in way.refs() {
                        _way.push(nid);
                    }
                    for (k,v) in way.tags(){
                        if k == "highway" {
                            _way_type = v.to_string();
                        }
                    }
                    way_refs.insert(way.id(),(_way,_way_type));
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

            for ref_node in &ref_nodes.0 {

                let value =nodes[&ref_node];
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

                way.push(nodes[&ref_node]);
            }


            let data = WayData{
                id: *key,
                way_points: way,
                max_points: Vector2D::new(x_max,y_max),
                min_points: Vector2D::new(x_min,y_min),
                highway_tag: ref_nodes.1.to_string(),
            };
            ways.insert(*key,data);
        }
        ways
    }
}

impl MotionElement for Map{
    fn draw_on(&self, frame: usize, canvas: &Canvas) -> Result<(), &'static str> {

        let scale = self.scale.get_frame(frame);
        let style_settings_map = self.settings.as_ref().unwrap();

        for (k,v) in self.data.map.iter() {

            let mut points = Vec::<Box<dyn attribute::Attribute<Vector2D<f32>>>>::new();
            for wp in v.way_points.iter() {
                let y = (self.data.center_point.x - wp.x) * scale as f64;
                let x = (wp.y - self.data.center_point.y) * scale as f64;
                points.push(Vector2D::new(x as f32, y as f32).into_bsa())
            }

            let highway_tag = &self.data.map[k].highway_tag;

            if(style_settings_map.way_settings.contains_key(highway_tag)){
                let style_settings_way = &style_settings_map.way_settings[highway_tag];

                Line {
                    position_offset: self.position.clone(),
                    start: 0f32.into_bsa(),
                    end: 1f32.into_bsa(),
                    width: style_settings_way.width.into_bsa(),
                    color: style_settings_way.color.into_bsa(),
                    stroke_caps: skia_safe::paint::Cap::Round,
                    is_antialias: true,
                    points,
                }.draw_on(frame, canvas)?;
            } else {
                Line {
                    position_offset: self.position.clone(),
                    start: 0f32.into_bsa(),
                    end: 1f32.into_bsa(),
                    width: 1f32.into_bsa(),
                    color: RGB{r:100, g: 100, b: 100}.into_bsa(),
                    stroke_caps: skia_safe::paint::Cap::Round,
                    is_antialias: true,
                    points,
                }.draw_on(frame, canvas)?;
            }



        };

        Ok(())
    }
}

impl Default for MapSelectionSettings{
    fn default()->Self{
        let map : HashMap<String,bool> = [
            (String::from_str("motorway").unwrap(),true),
            (String::from_str("trunk").unwrap(),true),
            (String::from_str("primary").unwrap(),true),
            (String::from_str("secondary").unwrap(),true),
            (String::from_str("tertiary").unwrap(),true),
            (String::from_str("unclassified").unwrap(),true),
            (String::from_str("residential").unwrap(),true),
        ].into_iter().collect();


        MapSelectionSettings{
            way_settings: map,
        }
    }
}

