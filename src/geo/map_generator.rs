use crate::geo::style::{Category, MapStyleSettings};
use crate::motion_graphics::attributes::attribute;
use crate::motion_graphics::attributes::type_extensions::InterpolationArithmetics;
use crate::motion_graphics::elements::element::DrawInfo;
use crate::motion_graphics::elements::Element as MotionElement;
use osmpbf::{Element, ElementReader};
use serde::{Deserialize, Serialize};
use skia_safe::Canvas;
use std::collections::{HashMap, LinkedList, VecDeque};
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
    pub(crate) fn from_ways(ways: &Vec<WayData>) -> Result<Vec<Self>, String> {

        let node = match ways.first() {
            Some(node) => node,
            None => return Ok(Vec::new()),
        };

        let mut first_node = match node.way_points.first(){
            Some(node) => node,
            None => return Ok(Vec::new()),
        };
        let mut last_node = match node.way_points.last(){
            Some(node) => node,
            None => return Ok(Vec::new()),
        };

        let mut prev_inversed: bool = false;

        let mut already_inserted = Vec::new();
        let mut temp_res = VecDeque::<RelationDrawOrder>::new();
        temp_res.push_back(RelationDrawOrder{
            index: 0,
            is_reversed: false,
        });

        let mut res = Vec::new();
        for _ in ways.iter(){
            for (i,way) in ways[1..].iter().enumerate() {
                let i = i + 1;
                let current_node_start = way.way_points.first().unwrap();
                let current_node_end = way.way_points.last().unwrap();

                if already_inserted.contains(&way.id) { continue; }

                if last_node == current_node_start{
                    //append after
                    let x = RelationDrawOrder{
                        index: i,
                        is_reversed: false,
                    };
                    temp_res.push_back(x);
                    last_node = current_node_end;
                    already_inserted.push(way.id.clone());
                    continue;
                }

                if first_node == current_node_end{
                    //append before
                    let x = RelationDrawOrder{
                        index: i,
                        is_reversed: false,
                    };
                    temp_res.push_front(x);
                    first_node = current_node_start;
                    already_inserted.push(way.id.clone());
                    continue;
                }

                if last_node == current_node_end {
                    //append after : reversed
                    let is_reversed= true ^ prev_inversed;
                    let x = RelationDrawOrder{
                        index: i,
                        is_reversed,
                    };
                    prev_inversed = is_reversed;
                    temp_res.push_back(x);

                    last_node = current_node_start;
                    already_inserted.push(way.id.clone());
                    continue;
                }

                if first_node == current_node_start {
                    //append before : reversed
                    let is_reversed= true ^ prev_inversed;
                    let x = RelationDrawOrder{
                        index: i,
                        is_reversed,
                    };
                    prev_inversed = is_reversed;
                    temp_res.push_front(x);

                    first_node = current_node_end;
                    already_inserted.push(way.id.clone());
                    continue;
                }

                let has_remaining_nodes = i < ways.len();
                if first_node == last_node && has_remaining_nodes
                {
                    for tr in &temp_res{
                        res.push(tr.clone());
                    }
                    let x = RelationDrawOrder{
                        index: i,
                        is_reversed: false,
                    };
                    temp_res.clear();
                    temp_res.push_back(x);

                    first_node = current_node_start;
                    last_node = current_node_end;
                }
            }
        }

        //add logic if relation consists of multiple areas instead of one big
        for tr in &temp_res{
            res.push(tr.clone());
        }

        Ok(res)
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
        self.draw_area_with_draw_order(frame, canvas, draw_info, parent, map_transform, &self.outer);
        self.draw_area(frame, canvas, draw_info, parent, map_transform, &self.inner);
        self.draw_area(frame, canvas, draw_info, parent, map_transform, &self.empty);
    }

    fn draw_area_with_draw_order(&self, frame: usize, canvas: &Canvas, draw_info: &DrawInfo, parent: &Map, map_transform: &MapTransform, area: &Vec<WayData>){
        let mut points = Vec::<Box<dyn attribute::Attribute<Vector2D<f32>>>>::new();

        //use draworder for index and direction.
        for xdo in &self.draw_orders{
            let way_data = &area[xdo.index];
            let mut way_points_list = way_data.way_points.clone();
            if xdo.is_reversed{
                way_points_list.reverse();
            }

            for wp in way_points_list {
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

    fn draw_area(&self, frame: usize, canvas: &Canvas, draw_info: &DrawInfo, parent: &Map, map_transform: &MapTransform, area: &Vec<WayData>){
        let mut points = Vec::<Box<dyn attribute::Attribute<Vector2D<f32>>>>::new();

        //use draworder for index and direction.
        for ways in area.iter(){
            for wp in &ways.way_points {
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
    fn draw_on_areas(&self, frame: usize, canvas: &Canvas, draw_info: &DrawInfo, parent: &Map, map_transform: &MapTransform){
        let settings = &parent.settings;

        let mut points = Vec::<Box<dyn attribute::Attribute<Vector2D<f32>>>>::new();
        for wp in self.way_points.iter() {
            let position_x = wp.x - map_transform.pos_geo.x as f64;
            let position_y = wp.y - map_transform.pos_geo.y as f64;

            let y = position_x * map_transform.scale as f64;
            let x = position_y * map_transform.scale as f64;

            let outer_frame_scale = 2f64;

            if  x > -draw_info.width as f64  * outer_frame_scale &&
                x < draw_info.width as f64   * outer_frame_scale &&
                y > -draw_info.height as f64 * outer_frame_scale &&
                y < draw_info.height as f64  * outer_frame_scale {
                points.push(Vector2D::new(x as f32, -y as f32).into_bsa())
            }
        }

        let _tag = &self.tag.clone();
        if _tag.is_some() {
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
                _ => { }
            }
        };
    }
    fn draw_on_ways(&self, frame: usize, canvas: &Canvas, draw_info: &DrawInfo, parent: &Map, map_transform: &MapTransform, category: Category){
        let settings = &parent.settings;

        let mut points = Vec::<Box<dyn attribute::Attribute<Vector2D<f32>>>>::new();
        for wp in self.way_points.iter() {
            let position_x = wp.x - map_transform.pos_geo.x as f64;
            let position_y = wp.y - map_transform.pos_geo.y as f64;

            let y = position_x * map_transform.scale as f64;
            let x = position_y * map_transform.scale as f64;

            let outer_frame_scale = 2f64;

            if  x > -draw_info.width as f64  * outer_frame_scale &&
                x < draw_info.width as f64   * outer_frame_scale &&
                y > -draw_info.height as f64 * outer_frame_scale &&
                y < draw_info.height as f64  * outer_frame_scale {
                points.push(Vector2D::new(x as f32, -y as f32).into_bsa())
            }
        }

        let _tag = &self.tag.clone();
        if _tag.is_some() {
            let _tag = _tag.clone().unwrap();
            if _tag.category == category {
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

        for area in &self.data.ways{
            area.draw_on_areas(frame, canvas, draw_info, &self, map_transform);
        }

        // for way in &self.data.ways{
        //     way.draw_on_ways(frame, canvas, draw_info, &self, map_transform, Category::Water);
        // }

        // for way in &self.data.ways{
        //     way.draw_on_ways(frame, canvas, draw_info, &self, map_transform, Category::Path);
        // }

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

