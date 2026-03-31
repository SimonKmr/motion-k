use crate::geo::style::{Category, MapStyleSettings};
use crate::motion_graphics::attributes::attribute;
use crate::motion_graphics::attributes::type_extensions::InterpolationArithmetics;
use crate::motion_graphics::elements::element::DrawInfo;
use crate::motion_graphics::elements::Element as MotionElement;
use osmpbf::{Element, ElementReader};
use serde::{Deserialize, Serialize};
use skia_safe::Canvas;
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;
use vector2d::Vector2D;

pub struct MapIO { }

pub struct Map{
    pub geo_position: Box<dyn attribute::Attribute<Vector2D<f32>>>,
    pub position: Box<dyn attribute::Attribute<Vector2D<f32>>>,
    pub scale: Box<dyn attribute::Attribute<f32>>,
    pub data: MapData,
    pub settings: Option<MapStyleSettings>,
}

#[derive(Serialize, Deserialize)]
pub struct MapData{
    pub relations: Vec<RelationData>,
    pub ways: Vec<WayData>,
    //pub points: Vec<Node>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RelationData{
    pub id: i64,
    pub tag: Option<Tag>,
    pub outer: Vec<WayData>,
    pub inner: Vec<WayData>,
    pub empty: Vec<WayData>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WayData{
    pub id: i64,
    pub tag: Option<Tag>,
    pub way_points: Vec<Node>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Node{
    pub id: i64,
    pub tag: Option<Tag>,
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Tag{
    pub value: String,
    pub category: Category,
}

impl Tag {
    pub(crate) fn new(category: Category, value: String) -> Tag {
        Tag{
            category,
            value,
        }
    }
}

impl MapIO {
    pub fn load(path: &String, settings: Option<MapStyleSettings>) -> MapData{

        let file_name_bin = format!("{path}.bin");
        let path_bin = Path::new(&path);

        return MapIO::import_osm(path,settings);

        // if Path::new(&path).exists(){
        //     return MapIO::import_binary(&file_name_bin)
        // } else {
        //     let res = MapIO::import_osm(path,settings);
        //     MapIO::export_binary(file_name_bin,&res);
        //     return res
        // }
    }

    pub fn import_osm(path: &String, settings: Option<MapStyleSettings>) -> MapData{

        let default_settings = MapStyleSettings::default();
        let settings = settings.as_ref().unwrap_or(&default_settings);

        let mut nodes = HashMap::<i64,Node>::new();
        let reader = ElementReader::from_path(&path).unwrap();
        reader.for_each(|element| {
            if let Element::Node(node) = element {
                let id = node.id();
                nodes.insert(id, Node{
                    id,
                    x: node.lat(),
                    y: node.lon(),
                    tag: None});
            }
            else if let Element::DenseNode(node) = element {
                let id = node.id();
                nodes.insert(id, Node{
                    id,
                    x: node.lat(),
                    y: node.lon(),
                    tag: None});
            } else { }

        }).unwrap();

        let mut _ways = HashMap::<i64,WayData>::new();
        let reader = ElementReader::from_path(&path).unwrap();
        reader.for_each(|element| {
            if let Element::Way(way) = element{
                let mut tag : Option<Tag> = None;
                for _tag in way.tags(){
                    if settings.filter_by_tag(_tag.0) && settings.filter_by_value(_tag.1){
                        tag = settings.map_tag_to_category(_tag.0,_tag.1);
                        break;
                    }
                }

                let mut way_nodes : Vec<Node> = Vec::new();
                for way_ref in way.refs(){
                    if nodes.contains_key(&way_ref){
                        way_nodes.push(nodes[&way_ref].clone());
                    }
                }

                let id = way.id();
                _ways.insert(id, WayData{ id, tag, way_points: way_nodes });
            }
        }).unwrap();

        let mut relations = Vec::<RelationData>::new();
        let reader = ElementReader::from_path(&path).unwrap();
        reader.for_each(|element| {
            if let Element::Relation(relation) = element {
                let id = relation.id();
                let mut tag : Option<Tag> = None;

                for _tag in relation.tags(){
                    if settings.filter_by_area_tag(_tag.0) && settings.filter_by_value(_tag.1){
                        tag = settings.map_tag_to_category(_tag.0,_tag.1);
                        break;
                    }
                }

                let mut inner = Vec::<WayData>::new();
                let mut outer = Vec::<WayData>::new();
                let mut empty = Vec::<WayData>::new();

                for member in relation.members(){
                    let index = member.member_id.clone();
                    if _ways.contains_key(&index){
                        let way = _ways[&index].clone();
                        match member.role() {
                            Ok("outer") => {outer.push(way);}
                            Ok("inner") => {inner.push(way);}
                            Ok("") => {empty.push(way)}
                            _ => {}
                        }
                    }
                }

                let relation = RelationData{
                    id,
                    tag,
                    outer,
                    inner,
                    empty,
                };
                relations.push(relation);
            }
        }).unwrap();

        let mut ways = Vec::<WayData>::new();
        for way in _ways{ ways.push(way.1.clone()); }

        MapData{
            relations,
            ways,
        }
    }

    fn import_binary(path: &String) -> Result<MapData, &'static str>{
        let bytes = std::fs::read(path).unwrap();
        let result : MapData = bincode::deserialize(bytes.as_slice()).unwrap();
        Ok(result)
    }

    fn export_binary(path: String, map_data: &MapData) {
        let bytes = bincode::serialize(map_data);
        std::fs::write(path, bytes.unwrap()).unwrap();
    }
}

impl RelationData {
    fn draw_on(&self, frame: usize, canvas: &Canvas, draw_info: &DrawInfo, parent: &Map){
        self.draw_area(frame, canvas, draw_info, parent, &self.outer);
        self.draw_area(frame, canvas, draw_info, parent, &self.inner);
        self.draw_area(frame, canvas, draw_info, parent, &self.empty);
    }

    fn draw_area(&self, frame: usize, canvas: &Canvas, draw_info: &DrawInfo, parent: &Map, area: &Vec<WayData>){
        let scale = parent.scale.get_frame(frame);
        let scale_mapped = scale.exp();

        let position = parent.position.get_frame(frame).into_ba();
        let geo_position = parent.geo_position.get_frame(frame);

        let mut points = Vec::<Box<dyn attribute::Attribute<Vector2D<f32>>>>::new();
        for way_data in area {
            for wp in way_data.way_points.iter() {
                let position_x = wp.x - geo_position.x as f64;
                let position_y = wp.y - geo_position.y as f64;

                let y = position_x * scale_mapped as f64;
                let x = position_y * scale_mapped as f64;

                if  x > -draw_info.width as f64 &&
                    x < draw_info.width as f64 &&
                    y > -draw_info.height as f64 &&
                    y < draw_info.height as f64 {

                    //-y is to flip the map
                    points.push(Vector2D::new(x as f32, -y as f32).into_bsa())
                }
            }
        }

        let default_settings = MapStyleSettings::default();
        let settings = parent.settings.as_ref().unwrap_or(&default_settings);
        let _tag = &self.tag.clone();
        if _tag.is_some(){
            let _tag = _tag.clone().unwrap();
            match _tag.category {
                Category::Area => {
                    if settings.area.contains_key(&_tag.value) {
                        let style = &settings.area[&_tag.value];
                        let res = style
                            .element(position, &points)
                            .draw_on(frame, canvas, draw_info);

                        match res {
                            Ok(_) => {}
                            Err(e) => {}
                        }
                    }
                }
                _ => { }
            }
        }
    }
}

impl WayData {
    fn draw_on(&self, frame: usize, canvas: &Canvas, draw_info: &DrawInfo, parent: &Map){
        let default_settings = MapStyleSettings::default();
        let settings = parent.settings.as_ref().unwrap_or(&default_settings);

        let scale = parent.scale.get_frame(frame);
        let scale_mapped = scale.exp();

        let position = parent.position.get_frame(frame).into_ba();
        let geo_position = parent.geo_position.get_frame(frame);

        let mut points = Vec::<Box<dyn attribute::Attribute<Vector2D<f32>>>>::new();
        for wp in self.way_points.iter() {
            let position_x = wp.x - geo_position.x as f64;
            let position_y = wp.y - geo_position.y as f64;

            let y = position_x * scale_mapped as f64;
            let x = position_y * scale_mapped as f64;

            if  x > -draw_info.width as f64 &&
                x < draw_info.width as f64 &&
                y > -draw_info.height as f64 &&
                y < draw_info.height as f64 {

                //-y is to flip the map
                points.push(Vector2D::new(x as f32, -y as f32).into_bsa())
            }
        }

        let _tag = &self.tag.clone();
        if _tag.is_some() {
            let _tag = _tag.clone().unwrap();
            match _tag.category {
                Category::Path => {
                    if settings.way.contains_key(&_tag.value) {
                        let style = &settings.way[&_tag.value];
                        let res = style
                            .element(position,&points)
                            .draw_on(frame, canvas, draw_info);

                        match res {
                            Ok(_) => {}
                            Err(e) => {}
                        }
                    }
                }
                Category::Area => {
                    if settings.area.contains_key(&_tag.value) {
                        let style = &settings.area[&_tag.value];
                        let res = style
                            .element(position, &points)
                            .draw_on(frame, canvas, draw_info);

                        match res {
                            Ok(_) => {}
                            Err(e) => {}
                        }
                    }
                }
                Category::Building => {
                    if settings.building.contains_key(&_tag.value) {
                        let style = &settings.building[&_tag.value];
                        let res = style
                            .element(position, &points)
                            .draw_on(frame, canvas, draw_info);

                        match res {
                            Ok(_) => {}
                            Err(e) => {}
                        }
                    }
                }
                _ => {}
            }
        };
    }
}

impl MotionElement for Map{
    fn draw_on(&self, frame: usize, canvas: &Canvas, draw_info: &DrawInfo) -> Result<(), &'static str> {
        let time = Instant::now();

        for relation in &self.data.relations{
            relation.draw_on(frame,  canvas, draw_info, &self);
        }

        for way in &self.data.ways{
            way.draw_on(frame,canvas,draw_info,&self);
        }

        let elapsed = time.elapsed();
        println!("Time elapsed is: {}", elapsed.as_millis());
        Ok(())
    }
}

