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

pub struct Map{
    pub geo_position: Box<dyn attribute::Attribute<Vector2D<f32>>>,
    pub position: Box<dyn attribute::Attribute<Vector2D<f32>>>,
    pub scale: Box<dyn attribute::Attribute<f32>>,
    pub data: MapData,
    pub settings: MapStyleSettings,
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
    pub draw_orders: Vec<RelationDrawOrder>,
    pub outer: Vec<WayData>,
    pub inner: Vec<WayData>,
    pub empty: Vec<WayData>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RelationDrawOrder{
    pub index: usize,
    pub is_reversed: bool,
}

impl RelationDrawOrder {
    pub(crate) fn from_ways(ways: &Vec<WayData>) -> Vec<Self> {
        let first_node = ways.first().unwrap().way_points.first().unwrap();
        let last_node = ways.first().unwrap().way_points.last().unwrap();

        for (i,way) in ways[1..].iter().enumerate() {
            let current_node_start = way.way_points.first().unwrap();
            let current_node_end = way.way_points.last().unwrap();

            if last_node == current_node_start{
                //append after
                //set last_node = current_node_end
                continue;
            }

            if first_node == current_node_end{
                //before
                //set first_node = current_node_start
                continue;
            }

            if first_node == current_node_start {
                //reversed
                //set first_node = current_node_end
                continue;
            }

            if last_node == current_node_end {
                //reversed
                //set last_node = current_node_start
                continue;
            }

            let has_remaining_nodes = i < ways.len() - 1;
            if first_node == last_node && has_remaining_nodes
            {
                //first_node =
            }
        }

        todo!()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WayData{
    pub id: i64,
    pub tag: Option<Tag>,
    pub way_points: Vec<Node>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Node{
    pub id: i64,
    pub tag: Option<Tag>,
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
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

impl RelationData {
    fn draw_on(&self, frame: usize, canvas: &Canvas, draw_info: &DrawInfo, parent: &Map, map_transform: &MapTransform){
        self.draw_area(frame, canvas, draw_info, parent, map_transform, &self.outer);
        //self.draw_area(frame, canvas, draw_info, parent, map_transform, &self.inner);
        //self.draw_area(frame, canvas, draw_info, parent, map_transform, &self.empty);
    }

    fn draw_area(&self, frame: usize, canvas: &Canvas, draw_info: &DrawInfo, parent: &Map, map_transform: &MapTransform, area: &Vec<WayData>){
        let mut points = Vec::<Box<dyn attribute::Attribute<Vector2D<f32>>>>::new();
        for way_data in area {
            for wp in way_data.way_points.iter() {
                let position_x = wp.x - map_transform.pos_geo.x as f64;
                let position_y = wp.y - map_transform.pos_geo.y as f64;

                let y = position_x * map_transform.scale as f64;
                let x = position_y * map_transform.scale as f64;

                if  x > -draw_info.width as f64 &&
                    x < draw_info.width as f64 &&
                    y > -draw_info.height as f64 &&
                    y < draw_info.height as f64 {
                    points.push(Vector2D::new(x as f32, -y as f32).into_bsa())
                }
            }
        }

        let settings = &parent.settings;
        let _tag = &self.tag.clone();
        if _tag.is_some(){
            let _tag = _tag.clone().unwrap();
            match _tag.category {
                Category::Area => {
                    if settings.area.contains_key(&_tag.value) {
                        let style = &settings.area[&_tag.value];
                        let res = style
                            .element(map_transform.pos.into_ba(), &points)
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
    fn draw_on(&self, frame: usize, canvas: &Canvas, draw_info: &DrawInfo, parent: &Map, map_transform: &MapTransform){
        let settings = &parent.settings;

        let mut points = Vec::<Box<dyn attribute::Attribute<Vector2D<f32>>>>::new();
        for wp in self.way_points.iter() {
            let position_x = wp.x - map_transform.pos_geo.x as f64;
            let position_y = wp.y - map_transform.pos_geo.y as f64;

            let y = position_x * map_transform.scale as f64;
            let x = position_y * map_transform.scale as f64;

            if  !(x > -draw_info.width as f64 &&
                x < draw_info.width as f64 &&
                y > -draw_info.height as f64 &&
                y < draw_info.height as f64) {
                return;
            }

            //-y is to flip the map
            points.push(Vector2D::new(x as f32, -y as f32).into_bsa())
        }

        let _tag = &self.tag.clone();
        if _tag.is_some() {
            let _tag = _tag.clone().unwrap();
            match _tag.category {
                Category::Path => {
                    if settings.way.contains_key(&_tag.value) {
                        let style = &settings.way[&_tag.value];
                        let res = style
                            .element(map_transform.pos.into_ba(),&points)
                            .draw_on(frame, canvas, draw_info);

                        match res {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                }
                Category::Area => {
                    if settings.area.contains_key(&_tag.value) {
                        let style = &settings.area[&_tag.value];
                        let res = style
                            .element(map_transform.pos.into_ba(), &points)
                            .draw_on(frame, canvas, draw_info);

                        match res {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                }
                Category::Building => {
                    if settings.building.contains_key(&_tag.value) {
                        let style = &settings.building[&_tag.value];
                        let res = style
                            .element(map_transform.pos.into_ba(), &points)
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

        let position = self.position.get_frame(frame);
        let scale = self.scale.get_frame(frame);
        let scale_mapped = scale.exp();
        let geo_position = self.geo_position.get_frame(frame);

        let map_transform = &MapTransform {scale: scale_mapped, pos_geo: geo_position, pos: position};

        for relation in &self.data.relations{
            relation.draw_on(frame,  canvas, draw_info, &self, map_transform);
        }

        for way in &self.data.ways{
            way.draw_on(frame,canvas,draw_info,&self, map_transform);
        }

        let elapsed = time.elapsed();
        println!("Time elapsed is: {}", elapsed.as_millis());
        Ok(())
    }
}

pub struct MapTransform {
    scale: f32,
    pos_geo: Vector2D<f32>,
    pos: Vector2D<f32>,
}

