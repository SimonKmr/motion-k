use crate::geo::style::{AreaStyleSettings, Category, MapStyleSettings, Style, WayStyleSettings};
use crate::motion_graphics::attributes::attribute;
use crate::motion_graphics::attributes::type_extensions::InterpolationArithmetics;
use crate::motion_graphics::elements::element::DrawInfo;
use crate::motion_graphics::elements::{shape, Element as MotionElement};
use serde::{Deserialize, Serialize};
use skia_safe::{Canvas, Point, RGB};
use std::collections::{HashMap, VecDeque};
use std::time::Instant;
use vector2d::Vector2D;
use crate::geo::pos_builder::{AreaPositionBuilder, OrderedAreaPositionBuilder, PositionBuilder, WayPositionBuilder};
pub(crate) use crate::geo::pos_builder::RelationDrawOrder;
use crate::motion_graphics::elements::rectangle::Rectangle;

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

impl MotionElement for Map{
    fn draw_on(&self, frame: usize, canvas: &Canvas, draw_info: &DrawInfo) -> Result<(), &'static str> {
        let time = Instant::now();

        let position = self.position.get_frame(frame);
        let scale = self.scale.get_frame(frame);
        let scale_mapped = scale.exp();
        let geo_position = self.geo_position.get_frame(frame);

        let map_transform = &MapTransform { scale, scale_mapped, pos_geo: geo_position, pos: position};

        //r: 160 , g: 142, b: 134
        //background
        Rectangle{
            position: Vector2D::new(0f32,0f32).into_bsa(),
            size: Vector2D::new(draw_info.width, draw_info.height).into_bsa(),
            color: RGB{r: 160,g: 142,b: 134}.into_bsa(),
            is_antialias: false,
        }.draw_on(frame, canvas, draw_info)?;

        for relation in &self.data.relations{

            let order = RelationDrawOrder::from_ways(&relation.outer);

            let pos_builder = OrderedAreaPositionBuilder{
                area: &relation.outer,
                transform: &map_transform,
                order: &order.unwrap(),
                draw_info,
            };
            Map::draw(frame, canvas, draw_info, map_transform, Category::Area, &self.settings.area, pos_builder, &relation.tag );
            //relation.draw(frame, canvas, draw_info, map_transform, &relation.inner, Category::Area, &self.settings.area, );
            //relation.draw(frame, canvas, draw_info, map_transform, &relation.empty, Category::Area, &self.settings.area, );
        }

        for area in &self.data.ways{
            let pos_builder = WayPositionBuilder{
                way_points: &area.way_points,
                transform: map_transform,
                draw_info: &draw_info,
            };
            Map::draw(frame, canvas, draw_info, map_transform, Category::Area, &self.settings.area, pos_builder, &area.tag );
        }

        for building in &self.data.ways{
            let pos_builder = WayPositionBuilder{
                way_points: &building.way_points,
                transform: map_transform,
                draw_info: &draw_info,
            };
            Map::draw(frame, canvas, draw_info, map_transform, Category::Building, &self.settings.building, pos_builder, &building.tag );
        }

        for way in &self.data.ways{
            let pos_builder = WayPositionBuilder{
                way_points: &way.way_points,
                transform: map_transform,
                draw_info: &draw_info,
            };
            Map::draw(frame, canvas, draw_info, map_transform, Category::Water, &self.settings.way, pos_builder, &way.tag );
        }

        for way in &self.data.ways{
            let pos_builder = WayPositionBuilder{
                way_points: &way.way_points,
                transform: map_transform,
                draw_info: &draw_info,
            };
            Map::draw(frame, canvas, draw_info, map_transform, Category::Path, &self.settings.way, pos_builder, &way.tag );
        }

        let elapsed = time.elapsed();
        println!("Time elapsed is: {}", elapsed.as_millis());
        Ok(())
    }
}

impl Map {
    fn draw(frame: usize, canvas: &Canvas, draw_info: &DrawInfo, map_transform: &MapTransform, category: Category, style_map: &HashMap<String, impl Style>, point_builder: impl PositionBuilder, tag: &Option<Tag>){
        let points = point_builder.build();

        let _tag = tag.clone();
        if _tag.is_none() { return; }

        let _tag = _tag.clone().unwrap();
        if _tag.category != category || !style_map.contains_key(&_tag.value) { return; }

        let style = &style_map[&_tag.value];

        if style.render_threshold() != None{
            if style.render_threshold().unwrap() > map_transform.scale {
                return;
            }
        }

        let res = style
            .element(map_transform.pos.into_ba(),points)
            .draw_on(frame, canvas, draw_info);

        match res {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

pub struct MapTransform {
    scale: f32,
    scale_mapped: f32,
    pos_geo: Vector2D<f32>,
    pos: Vector2D<f32>,
}

impl MapTransform {
    pub fn apply(&self, node : &Node) -> (f64,f64){
        let position_x = node.x - self.pos_geo.x as f64;
        let position_y = node.y - self.pos_geo.y as f64;

        let x = position_x * self.scale_mapped as f64;
        let y = position_y * self.scale_mapped as f64;
        (x,y)
    }

    pub fn is_on_screen(x: f64, y: f64, draw_info: &DrawInfo, range: f64) -> bool {
        x > -draw_info.width as f64  * range &&
            x < draw_info.width as f64   * range &&
            y > -draw_info.height as f64 * range &&
            y < draw_info.height as f64  * range
    }
}

impl Tag {
    pub(crate) fn new(category: Category, value: String) -> Tag {
        Tag{
            category,
            value,
        }
    }
}